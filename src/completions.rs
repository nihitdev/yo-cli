use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

impl Shell {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "fish" => Some(Self::Fish),
            "powershell" | "pwsh" => Some(Self::Powershell),
            _ => None,
        }
    }
}

pub fn print(shell: Shell) -> io::Result<()> {
    io::stdout().write_all(script(shell).as_bytes())
}

pub fn script(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => BASH,
        Shell::Zsh => ZSH,
        Shell::Fish => FISH,
        Shell::Powershell => POWERSHELL,
    }
}

#[cfg(test)]
const COMMANDS: &str =
    "init config doctor edit fetch status project session tip tips completions version help";
#[cfg(test)]
const DISPLAY_OPTIONS: &str = "--json --no-art --plain --theme";
#[cfg(test)]
const RUN_OPTIONS: &str = "--fast --no-art --plain --name --theme -h --help -V --version";

const BASH: &str = r#"_yoo() {
  local current previous command
  COMPREPLY=()
  current="${COMP_WORDS[COMP_CWORD]}"
  previous="${COMP_WORDS[COMP_CWORD-1]}"
  command="${COMP_WORDS[1]}"

  if [[ "$previous" == "completions" ]]; then
    COMPREPLY=( $(compgen -W "bash zsh fish powershell" -- "$current") )
    return
  fi

  if [[ "$previous" == "--theme" ]]; then
    COMPREPLY=( $(compgen -W "neon ocean mono dracula tokyo-night gruvbox nord rose-pine catppuccin" -- "$current") )
    return
  fi

  case "$command" in
    fetch|status|project)
      COMPREPLY=( $(compgen -W "--json --no-art --plain --theme" -- "$current") ) ;;
    session)
      COMPREPLY=( $(compgen -W "--minutes" -- "$current") ) ;;
    *)
      COMPREPLY=( $(compgen -W "init config doctor edit fetch status project session tip tips completions version help --fast --no-art --plain --name --theme -h --help -V --version" -- "$current") ) ;;
  esac
}
complete -F _yoo yoo
"#;

const ZSH: &str = r#"#compdef yoo

_yoo() {
  local -a commands themes shells
  commands=(
    'init:Create the default config and sample tip pack'
    'config:Print the config file location'
    'doctor:Check tools and project setup'
    'edit:Open the current directory in an editor'
    'fetch:Show environment and project information'
    'status:Alias for fetch'
    'project:Show a structured project overview'
    'session:Start a coding-session timer'
    'tip:Print a random tip'
    'tips:List installed tip packs'
    'completions:Generate shell completions'
    'version:Print version'
    'help:Print help'
  )
  themes=(neon ocean mono dracula tokyo-night gruvbox nord rose-pine catppuccin)
  shells=(bash zsh fish powershell)

  _arguments -C \
    '--fast[skip the typewriter animation]' \
    '--no-art[hide the ASCII logo]' \
    '--plain[disable ANSI colours]' \
    '--name[override the profile name]:name:' \
    '--theme[override the theme]:theme:($themes)' \
    '1:command:->command' \
    '*::argument:->args'

  case "$state" in
    command) _describe 'command' commands ;;
    args)
      case "$words[2]" in
        fetch|status|project)
          _arguments '--json[machine-readable JSON]' '--no-art[hide art]' '--plain[disable colours]' '--theme[override theme]:theme:($themes)' ;;
        session) _arguments '--minutes[session length]:minutes:' '1:minutes:' ;;
        edit) _arguments '--editor[use a specific editor]:editor:_command_names' ;;
        completions) _values 'shell' $shells ;;
      esac ;;
  esac
}

_yoo "$@"
"#;

const FISH: &str = r#"complete -c yoo -f
complete -c yoo -n '__fish_use_subcommand' -a init -d 'Create the default config and sample tip pack'
complete -c yoo -n '__fish_use_subcommand' -a config -d 'Print the config file location'
complete -c yoo -n '__fish_use_subcommand' -a doctor -d 'Check tools and project setup'
complete -c yoo -n '__fish_use_subcommand' -a edit -d 'Open the current directory in an editor'
complete -c yoo -n '__fish_use_subcommand' -a fetch -d 'Show environment and project information'
complete -c yoo -n '__fish_use_subcommand' -a status -d 'Alias for fetch'
complete -c yoo -n '__fish_use_subcommand' -a project -d 'Show a structured project overview'
complete -c yoo -n '__fish_use_subcommand' -a session -d 'Start a coding-session timer'
complete -c yoo -n '__fish_use_subcommand' -a tip -d 'Print a random tip'
complete -c yoo -n '__fish_use_subcommand' -a tips -d 'List installed tip packs'
complete -c yoo -n '__fish_use_subcommand' -a completions -d 'Generate shell completions'
complete -c yoo -n '__fish_use_subcommand' -a version -d 'Print version'
complete -c yoo -n '__fish_use_subcommand' -a help -d 'Print help'
complete -c yoo -n '__fish_seen_subcommand_from completions' -a 'bash zsh fish powershell'
complete -c yoo -n '__fish_seen_subcommand_from fetch status project' -l json -d 'Print machine-readable JSON'
complete -c yoo -n '__fish_seen_subcommand_from fetch status project' -l no-art -d 'Hide the ASCII logo'
complete -c yoo -n '__fish_seen_subcommand_from fetch status project' -l plain -d 'Disable ANSI colours'
complete -c yoo -n '__fish_seen_subcommand_from fetch status project' -l theme -xa 'neon ocean mono dracula tokyo-night gruvbox nord rose-pine catppuccin'
complete -c yoo -n '__fish_seen_subcommand_from session' -l minutes -d 'Session length in minutes'
complete -c yoo -n '__fish_seen_subcommand_from edit' -l editor -r -d 'Use a specific editor'
complete -c yoo -n 'not __fish_seen_subcommand_from init config doctor edit fetch status project session tip tips completions version help' -l fast -d 'Skip the typewriter animation'
complete -c yoo -n 'not __fish_seen_subcommand_from init config doctor edit fetch status project session tip tips completions version help' -l no-art -d 'Hide the ASCII logo'
complete -c yoo -n 'not __fish_seen_subcommand_from init config doctor edit fetch status project session tip tips completions version help' -l plain -d 'Disable ANSI colours'
complete -c yoo -n 'not __fish_seen_subcommand_from init config doctor edit fetch status project session tip tips completions version help' -l name -r -d 'Override the profile name'
complete -c yoo -n 'not __fish_seen_subcommand_from init config doctor edit fetch status project session tip tips completions version help' -l theme -xa 'neon ocean mono dracula tokyo-night gruvbox nord rose-pine catppuccin'
complete -c yoo -s h -l help -d 'Print help'
complete -c yoo -s V -l version -d 'Print version'
"#;

const POWERSHELL: &str = r#"Register-ArgumentCompleter -Native -CommandName yoo -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)

  $commands = 'init','config','doctor','edit','fetch','status','project','session','tip','tips','completions','version','help'
  $themes = 'neon','ocean','mono','dracula','tokyo-night','gruvbox','nord','rose-pine','catppuccin'
  $shells = 'bash','zsh','fish','powershell'
  $tokens = @($commandAst.CommandElements | ForEach-Object { $_.Extent.Text })
  $previous = if ($tokens.Count -gt 1) { $tokens[-2] } else { '' }
  $command = if ($tokens.Count -gt 1) { $tokens[1] } else { '' }

  if ($previous -eq 'completions') { $values = $shells }
  elseif ($previous -eq '--theme') { $values = $themes }
  elseif ($command -in @('fetch','status','project')) { $values = '--json','--no-art','--plain','--theme' }
  elseif ($command -eq 'session') { $values = '--minutes' }
  elseif ($command -eq 'edit') { $values = '--editor' }
  else { $values = $commands + @('--fast','--no-art','--plain','--name','--theme','-h','--help','-V','--version') }

  $values | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
    [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
  }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_completion_contains_all_commands() {
        for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Powershell] {
            let generated = script(shell);
            for command in COMMANDS.split_whitespace() {
                assert!(
                    generated.contains(command),
                    "{shell:?} is missing {command}"
                );
            }
        }
    }

    #[test]
    fn completion_option_lists_match_the_cli() {
        for option in DISPLAY_OPTIONS.split_whitespace() {
            assert!(BASH.contains(option));
            assert!(POWERSHELL.contains(option));
        }
        for option in RUN_OPTIONS.split_whitespace() {
            assert!(BASH.contains(option));
            assert!(POWERSHELL.contains(option));
        }
    }
}
