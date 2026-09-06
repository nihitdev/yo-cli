_yoo() {
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
