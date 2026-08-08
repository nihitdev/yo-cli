use crate::timer;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RunOptions {
    pub fast: bool,
    pub no_art: bool,
    pub plain: bool,
    pub name: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FetchOptions {
    pub no_art: bool,
    pub plain: bool,
    pub theme: Option<String>,
    pub json: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectOptions {
    pub no_art: bool,
    pub plain: bool,
    pub theme: Option<String>,
    pub json: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionOptions {
    pub minutes: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EditOptions {
    pub editor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Run(RunOptions),
    Init,
    ConfigPath,
    Doctor,
    Edit(EditOptions),
    Fetch(FetchOptions),
    Project(ProjectOptions),
    Session(SessionOptions),
    Tip(Option<String>),
    Tips,
    Completions(crate::completions::Shell),
    Help,
    Version,
}

pub fn parse(arguments: &[String]) -> Result<Command, String> {
    if arguments.is_empty() {
        return Ok(Command::Run(RunOptions::default()));
    }

    match arguments[0].as_str() {
        "help" | "--help" | "-h" => require_standalone(arguments, Command::Help),
        "init" | "--init" => require_standalone(arguments, Command::Init),
        "config" | "--config" => require_standalone(arguments, Command::ConfigPath),
        "doctor" => require_standalone(arguments, Command::Doctor),
        "edit" => parse_edit(&arguments[1..]),
        "fetch" | "status" => parse_fetch(&arguments[1..]),
        "project" => parse_project(&arguments[1..]),
        "tips" => require_standalone(arguments, Command::Tips),
        "completions" => parse_completions(&arguments[1..]),
        "tip" => parse_tip(&arguments[1..]),
        "session" => parse_session(&arguments[1..]),
        "version" | "--version" | "-V" => require_standalone(arguments, Command::Version),
        _ => parse_run(arguments),
    }
}

fn parse_edit(arguments: &[String]) -> Result<Command, String> {
    match arguments {
        [] => Ok(Command::Edit(EditOptions::default())),
        [flag, editor] if flag == "--editor" => Ok(Command::Edit(EditOptions {
            editor: Some(required_value(arguments, 1, "--editor")?),
        })),
        _ => Err("usage: yoo edit [--editor <EDITOR>]".to_owned()),
    }
}

fn parse_completions(arguments: &[String]) -> Result<Command, String> {
    let [shell] = arguments else {
        return Err("usage: yoo completions <bash|zsh|fish|powershell>".to_owned());
    };

    crate::completions::Shell::parse(shell)
        .map(Command::Completions)
        .ok_or_else(|| format!("unsupported shell `{shell}`; use bash, zsh, fish, or powershell"))
}

fn parse_run(arguments: &[String]) -> Result<Command, String> {
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

fn parse_fetch(arguments: &[String]) -> Result<Command, String> {
    let mut options = FetchOptions::default();
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--no-art" => options.no_art = true,
            "--plain" => options.plain = true,
            "--json" => options.json = true,
            "--theme" => {
                index += 1;
                options.theme = Some(required_value(arguments, index, "--theme")?);
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown fetch option `{value}`"));
            }
            value => return Err(format!("unknown fetch argument `{value}`")),
        }

        index += 1;
    }

    if options.json && (options.no_art || options.plain || options.theme.is_some()) {
        return Err("`--json` cannot be combined with display options".to_owned());
    }

    Ok(Command::Fetch(options))
}

fn parse_project(arguments: &[String]) -> Result<Command, String> {
    let mut options = ProjectOptions::default();
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--no-art" => options.no_art = true,
            "--plain" => options.plain = true,
            "--json" => options.json = true,
            "--theme" => {
                index += 1;
                options.theme = Some(required_value(arguments, index, "--theme")?);
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown project option `{value}`"));
            }
            value => return Err(format!("unknown project argument `{value}`")),
        }

        index += 1;
    }

    if options.json && (options.no_art || options.plain || options.theme.is_some()) {
        return Err("`--json` cannot be combined with display options".to_owned());
    }

    Ok(Command::Project(options))
}

fn parse_session(arguments: &[String]) -> Result<Command, String> {
    if arguments.is_empty() {
        return Ok(Command::Session(SessionOptions::default()));
    }

    if arguments.len() == 1 {
        return Ok(Command::Session(SessionOptions {
            minutes: Some(parse_minutes(&arguments[0])?),
        }));
    }

    if arguments.len() == 2 && arguments[0] == "--minutes" {
        return Ok(Command::Session(SessionOptions {
            minutes: Some(parse_minutes(&arguments[1])?),
        }));
    }

    Err("usage: yoo session [MINUTES] or yoo session --minutes <MINUTES>".to_owned())
}

fn parse_tip(arguments: &[String]) -> Result<Command, String> {
    match arguments {
        [] => Ok(Command::Tip(None)),
        [pack] if !pack.starts_with('-') => Ok(Command::Tip(Some(pack.trim().to_owned()))),
        _ => Err("usage: yoo tip [PACK]".to_owned()),
    }
}

fn parse_minutes(value: &str) -> Result<u64, String> {
    let minutes = value.parse::<u64>().map_err(|_| {
        format!(
            "minutes must be a whole number between {} and {}",
            timer::MIN_MINUTES,
            timer::MAX_MINUTES
        )
    })?;

    if !(timer::MIN_MINUTES..=timer::MAX_MINUTES).contains(&minutes) {
        return Err(format!(
            "minutes must be between {} and {}",
            timer::MIN_MINUTES,
            timer::MAX_MINUTES
        ));
    }

    Ok(minutes)
}

fn require_standalone(arguments: &[String], command: Command) -> Result<Command, String> {
    if arguments.len() == 1 {
        Ok(command)
    } else {
        Err(format!(
            "`{}` cannot be combined with other options",
            arguments[0]
        ))
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
    r#"yoo — local project and development environment information

USAGE:
  yoo [OPTIONS]
  yoo <COMMAND>

COMMANDS:
  init                    Create the default TOML config and a sample community tip pack
  config                  Print the TOML config file location
  doctor                  Check Rust, Cargo, Git, config, and current-project setup
  edit [OPTIONS]          Open the current directory in your preferred editor
  fetch [OPTIONS]         Show developer environment and current-project information
  status [OPTIONS]        Alias for `yoo fetch`
  project [OPTIONS]       Show a structured overview of the current project
  session [MINUTES]       Start a local coding-session timer (default comes from config)
  tip [PACK]              Print one random tip; PACK defaults to your configured pack
  tips                    List built-in and locally installed community tip packs
  completions <SHELL>     Generate completions for Bash, Zsh, Fish, or PowerShell
  version                 Print version
  help                    Print this help message

FETCH / PROJECT OPTIONS:
  --json                  Print machine-readable JSON with no decoration
  --no-art                Hide the ASCII logo for this run
  --plain                 Disable ANSI colours for this run
  --theme <THEME>         Override the theme for this run only

EDIT OPTIONS:
  --editor <EDITOR>       Use a specific editor instead of automatic detection

RUN OPTIONS:
  --fast                  Skip the typewriter animation
  --no-art                Hide the ASCII logo for this run
  --plain                 Disable ANSI colours for this run
  --name <NAME>           Use a name for this run only
  --theme <THEME>         Override the theme for this run only
  -h, --help              Print help
  -V, --version           Print version

THEMES:
  neon, ocean, mono, dracula, tokyo-night, gruvbox, nord, rose-pine, catppuccin

EXAMPLES:
  yoo
  yoo --fast --theme tokyo-night
  yoo doctor
  yoo edit
  yoo edit --editor cursor
  yoo fetch
  yoo fetch --json
  yoo project
  yoo project --json
  yoo status --plain
  yoo session 45
  yoo tip rust
  yoo tips
  yoo completions bash
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
    fn parses_fetch_options() {
        let arguments = values(&["fetch", "--no-art", "--plain", "--theme", "nord"]);
        assert_eq!(
            parse(&arguments),
            Ok(Command::Fetch(FetchOptions {
                no_art: true,
                plain: true,
                theme: Some("nord".to_owned()),
                json: false,
            }))
        );
    }

    #[test]
    fn parses_fetch_json() {
        let arguments = values(&["fetch", "--json"]);
        assert_eq!(
            parse(&arguments),
            Ok(Command::Fetch(FetchOptions {
                json: true,
                ..FetchOptions::default()
            }))
        );
    }

    #[test]
    fn parses_project_json() {
        let arguments = values(&["project", "--json"]);
        assert_eq!(
            parse(&arguments),
            Ok(Command::Project(ProjectOptions {
                json: true,
                ..ProjectOptions::default()
            }))
        );
    }

    #[test]
    fn rejects_json_with_display_options() {
        let arguments = values(&["fetch", "--json", "--plain"]);
        assert!(parse(&arguments).is_err());
    }

    #[test]
    fn rejects_project_json_with_display_options() {
        let arguments = values(&["project", "--json", "--no-art"]);
        assert!(parse(&arguments).is_err());
    }

    #[test]
    fn parses_session_minutes() {
        let arguments = values(&["session", "45"]);
        assert_eq!(
            parse(&arguments),
            Ok(Command::Session(SessionOptions { minutes: Some(45) }))
        );
    }

    #[test]
    fn parses_edit_options() {
        assert_eq!(
            parse(&values(&["edit"])),
            Ok(Command::Edit(EditOptions::default()))
        );
        assert_eq!(
            parse(&values(&["edit", "--editor", "cursor"])),
            Ok(Command::Edit(EditOptions {
                editor: Some("cursor".to_owned()),
            }))
        );
        assert!(parse(&values(&["edit", "cursor"])).is_err());
    }

    #[test]
    fn rejects_invalid_session_minutes() {
        let arguments = values(&["session", "0"]);
        assert!(parse(&arguments).is_err());
    }

    #[test]
    fn parses_tip_pack() {
        let arguments = values(&["tip", "rust"]);
        assert_eq!(parse(&arguments), Ok(Command::Tip(Some("rust".to_owned()))));
    }

    #[test]
    fn parses_version_command() {
        assert_eq!(parse(&values(&["version"])), Ok(Command::Version));
        assert_eq!(parse(&values(&["--version"])), Ok(Command::Version));
    }

    #[test]
    fn parses_completion_shell() {
        assert_eq!(
            parse(&values(&["completions", "powershell"])),
            Ok(Command::Completions(crate::completions::Shell::Powershell))
        );
        assert!(parse(&values(&["completions", "unknown"])).is_err());
    }

    #[test]
    fn rejects_unknown_options() {
        let arguments = values(&["--turbo"]);
        assert!(parse(&arguments).is_err());
    }
}
