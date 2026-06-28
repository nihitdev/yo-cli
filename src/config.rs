use std::{
    env,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::ui::Theme;

pub const DEFAULT_CONFIG: &str = r#"# yoo configuration
# Edit this file by hand, or use yq if YAML is your thing.

version: 1

profile:
  name: developer

appearance:
  theme: neon
  ascii: true
  colors: true
  typing_speed_ms: 12

git:
  show_branch: true
  show_status: true

tips:
  enabled: true
  pack: general

hydration:
  enabled: true

session:
  default_minutes: 25
  show_complete_message: true
"#;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct Config {
    pub version: u8,
    pub profile: ProfileConfig,
    pub appearance: AppearanceConfig,
    pub git: GitConfig,
    pub tips: TipsConfig,
    pub hydration: HydrationConfig,
    pub session: SessionConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ProfileConfig {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub theme: String,
    pub ascii: bool,
    pub colors: bool,
    pub typing_speed_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct GitConfig {
    pub show_branch: bool,
    pub show_status: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct TipsConfig {
    pub enabled: bool,
    pub pack: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct HydrationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct SessionConfig {
    pub default_minutes: u64,
    pub show_complete_message: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            profile: ProfileConfig::default(),
            appearance: AppearanceConfig::default(),
            git: GitConfig::default(),
            tips: TipsConfig::default(),
            hydration: HydrationConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            name: "developer".to_owned(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "neon".to_owned(),
            ascii: true,
            colors: true,
            typing_speed_ms: 12,
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            show_branch: true,
            show_status: true,
        }
    }
}

impl Default for TipsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pack: "general".to_owned(),
        }
    }
}

impl Default for HydrationConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            default_minutes: 25,
            show_complete_message: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteResult {
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

pub fn config_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(app_data) = env::var_os("APPDATA") {
            return PathBuf::from(app_data).join("yoo");
        }
    }

    if cfg!(target_os = "macos") {
        if let Some(home) = home_dir() {
            return home.join("Library").join("Application Support").join("yoo");
        }
    }

    if let Some(xdg_config) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg_config).join("yoo");
    }

    if let Some(home) = home_dir() {
        return home.join(".config").join("yoo");
    }

    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".yoo")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.yaml")
}

pub fn tip_packs_dir() -> PathBuf {
    config_dir().join("tips")
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

pub fn write_default() -> io::Result<WriteResult> {
    let path = config_path();

    if path.exists() {
        return Ok(WriteResult::AlreadyExists);
    }

    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    fs::write(path, DEFAULT_CONFIG)?;

    Ok(WriteResult::Created)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

fn parse(contents: &str) -> Result<Config, ConfigError> {
    let config = serde_yaml::from_str::<Config>(contents)
        .map_err(|error| ConfigError::new(format!("invalid YAML config: {error}")))?;

    validate(&config)?;
    Ok(config)
}

fn validate(config: &Config) -> Result<(), ConfigError> {
    if config.version != 1 {
        return Err(ConfigError::new("version must be 1"));
    }

    if config.profile.name.trim().is_empty() {
        return Err(ConfigError::new("profile.name cannot be empty"));
    }

    if Theme::parse(&config.appearance.theme).is_none() {
        return Err(ConfigError::new(format!(
            "appearance.theme must be one of: {}",
            Theme::names().join(", ")
        )));
    }

    if config.appearance.typing_speed_ms > 250 {
        return Err(ConfigError::new(
            "appearance.typing_speed_ms must be between 0 and 250",
        ));
    }

    if config.tips.pack.trim().is_empty() {
        return Err(ConfigError::new("tips.pack cannot be empty"));
    }

    if !(1..=480).contains(&config.session.default_minutes) {
        return Err(ConfigError::new(
            "session.default_minutes must be between 1 and 480",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_yaml_config() {
        let config = parse(
            r#"
version: 1
profile:
  name: Nihit
appearance:
  theme: ocean
  ascii: false
  colors: true
  typing_speed_ms: 0
git:
  show_branch: true
  show_status: false
tips:
  enabled: true
  pack: rust
hydration:
  enabled: false
session:
  default_minutes: 45
  show_complete_message: true
"#,
        )
        .expect("config should parse");

        assert_eq!(config.profile.name, "Nihit");
        assert_eq!(config.appearance.theme, "ocean");
        assert!(!config.appearance.ascii);
        assert_eq!(config.session.default_minutes, 45);
    }

    #[test]
    fn defaults_fill_missing_sections() {
        let config = parse("version: 1\nprofile:\n  name: Nihit\n").expect("config should parse");
        assert_eq!(config.profile.name, "Nihit");
        assert_eq!(config.tips.pack, "general");
    }

    #[test]
    fn rejects_an_unknown_theme() {
        let error = parse("appearance:\n  theme: purple\n").expect_err("theme should be rejected");
        assert!(error.to_string().contains("appearance.theme"));
    }
}
