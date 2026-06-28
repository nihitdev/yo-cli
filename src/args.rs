#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RunOptions {
    pub fast: bool,
    pub no_art: bool,
    pub plain: bool,
    pub name: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Run(RunOptions),
    Init,
    ConfigPath,
    Help,
    Version,
}

pub fn parse(arguments: &[String]) -> Result<Command, String> {
    if arguments.is_empty() {
        return Ok(Command::Run(RunOptions::default()));
    }

    if matches!(arguments[0].as_str(), "help" | "--help" | "-h") {
        return require_standalone(arguments, Command::Help);
    }

    if matches!(arguments[0].as_str(), "init" | "--init") {
        return require_standalone(arguments, Command::Init);
    }

    if matches!(arguments[0].as_str(), "config" | "--config") {
        return require_standalone(arguments, Command::ConfigPath);
    }

    if matches!(arguments[0].as_str(), "--version" | "-V") {
        return require_standalone(arguments, Command::Version);
    }

    let mut options = RunOptions::default();
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--fast" => options.fast = true,
            "--no-art" => options.no_art = true,
            "--plain" => options.plain = true,
            "--name" => {
                index += 1;
                options.name = Some(required_value(arguments, index, "--name")?);
            }
            "--theme" => {
                index += 1;
                options.theme = Some(required_value(arguments, index, "--theme")?);
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => return Err(format!("unknown command `{value}`")),
        }

        index += 1;
    }

    Ok(Command::Run(options))
}

fn require_standalone(arguments: &[String], command: Command) -> Result<Command, String> {
    if arguments.len() == 1 {
        Ok(command)
    } else {
        Err(format!("`{}` cannot be combined with other options", arguments[0]))
    }
}

fn required_value(arguments: &[String], index: usize, flag: &str) -> Result<String, String> {
    let value = arguments
        .get(index)
        .ok_or_else(|| format!("{flag} needs a value"))?
        .trim()
        .to_owned();

    if value.is_empty() {
        return Err(format!("{flag} cannot be empty"));
    }

    Ok(value)
}

pub fn help_text() -> &'static str {
    r#"yoo — a tiny developer companion

USAGE:
  yoo [OPTIONS]
  yoo <COMMAND>

COMMANDS:
  init              Create a starter config file
  config            Print the config file location
  help              Print this help message

OPTIONS:
  --fast            Skip the typewriter animation
  --no-art          Hide the ASCII logo for this run
  --plain           Disable ANSI colours for this run
  --name <NAME>     Use a name for this run only
  --theme <THEME>   neon, ocean, or mono for this run only
  -h, --help        Print help
  -V, --version     Print version

EXAMPLES:
  yoo
  yoo --fast --theme ocean
  yoo --name Nihit --no-art
  yoo init
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    #[test]
    fn no_arguments_runs_with_defaults() {
        assert_eq!(parse(&[]), Ok(Command::Run(RunOptions::default())));
    }

    #[test]
    fn parses_run_options() {
        let arguments = values(&["--fast", "--no-art", "--name", "Nihit", "--theme", "ocean"]);
        let command = parse(&arguments).expect("options should parse");

        assert_eq!(
            command,
            Command::Run(RunOptions {
                fast: true,
                no_art: true,
                plain: false,
                name: Some("Nihit".to_owned()),
                theme: Some("ocean".to_owned()),
            })
        );
    }

    #[test]
    fn rejects_unknown_options() {
        let arguments = values(&["--turbo"]);
        assert!(parse(&arguments).is_err());
    }

    #[test]
    fn init_must_be_standalone() {
        let arguments = values(&["init", "--fast"]);
        assert!(parse(&arguments).is_err());
    }
}
