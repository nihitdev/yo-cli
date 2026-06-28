use std::{
    env,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

pub const DEFAULT_CONFIG: &str = r#"# yoo configuration
# This is intentionally small so you can edit it by hand.

name = "developer"
theme = "neon"
typing_speed_ms = 12
show_art = true
show_git = true
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub theme: String,
    pub typing_speed_ms: u64,
    pub show_art: bool,
    pub show_git: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "developer".to_owned(),
            theme: "neon".to_owned(),
            typing_speed_ms: 12,
            show_art: true,
            show_git: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitResult {
    Created,
    AlreadyExists,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ConfigError {}

pub fn config_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(app_data) = env::var_os("APPDATA") {
            return PathBuf::from(app_data).join("yoo").join("config.toml");
        }
    } else if let Some(xdg_config) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg_config).join("yoo").join("config.toml");
    }

    if let Some(home) = home_dir() {
        return home.join(".config").join("yoo").join("config.toml");
    }

    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".yoo")
        .join("config.toml")
}

pub fn load() -> Result<Config, ConfigError> {
    let path = config_path();

    match fs::read_to_string(&path) {
        Ok(contents) => parse(&contents),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Config::default()),
        Err(error) => Err(ConfigError::new(format!(
            "could not read {}: {error}",
            path.display()
        ))),
    }
}

pub fn write_default() -> io::Result<InitResult> {
    let path = config_path();

    if path.exists() {
        return Ok(InitResult::AlreadyExists);
    }

    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    fs::write(path, DEFAULT_CONFIG)?;

    Ok(InitResult::Created)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

fn parse(contents: &str) -> Result<Config, ConfigError> {
    let mut config = Config::default();

    for (line_index, raw_line) in contents.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();

        if line.is_empty() {
            continue;
        }

        let (key, value) = line.split_once('=').ok_or_else(|| {
            ConfigError::new(format!("line {} must use key = value", line_index + 1))
        })?;

        let key = key.trim();
        let value = value.trim().trim_matches('"');

        match key {
            "name" => {
                if value.is_empty() {
                    return Err(ConfigError::new("name cannot be empty"));
                }
                config.name = value.to_owned();
            }
            "theme" => config.theme = value.to_lowercase(),
            "typing_speed_ms" => {
                config.typing_speed_ms = value.parse::<u64>().map_err(|_| {
                    ConfigError::new("typing_speed_ms must be a whole number between 0 and 250")
                })?;
            }
            "show_art" => {
                config.show_art = value
                    .parse::<bool>()
                    .map_err(|_| ConfigError::new("show_art must be true or false"))?;
            }
            "show_git" => {
                config.show_git = value
                    .parse::<bool>()
                    .map_err(|_| ConfigError::new("show_git must be true or false"))?;
            }
            _ => {
                // Unknown keys are ignored so future versions can add settings without breaking old ones.
            }
        }
    }

    validate(&config)?;
    Ok(config)
}

fn validate(config: &Config) -> Result<(), ConfigError> {
    if !matches!(config.theme.as_str(), "neon" | "ocean" | "mono") {
        return Err(ConfigError::new("theme must be one of: neon, ocean, mono"));
    }

    if config.typing_speed_ms > 250 {
        return Err(ConfigError::new(
            "typing_speed_ms must be between 0 and 250",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_config() {
        let config = parse(
            r#"
                name = "Nihit"
                theme = "ocean"
                typing_speed_ms = 0
                show_art = false
                show_git = true
            "#,
        )
        .expect("config should parse");

        assert_eq!(config.name, "Nihit");
        assert_eq!(config.theme, "ocean");
        assert_eq!(config.typing_speed_ms, 0);
        assert!(!config.show_art);
        assert!(config.show_git);
    }

    #[test]
    fn rejects_an_unknown_theme() {
        let error = parse("theme = \"purple\"").expect_err("theme should be rejected");
        assert!(error.to_string().contains("neon, ocean, mono"));
    }
}
