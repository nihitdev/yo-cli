# Shell completions

`yoo completions <SHELL>` prints a completion script generated for the installed CLI version. Regenerate the script after upgrading `yoo` so new commands and options become available.

Supported values are `bash`, `zsh`, `fish`, and `powershell` (`pwsh` is accepted as an alias).

## Bash

Load completions for the current session:

```bash
source <(yoo completions bash)
```

Install them persistently for user-level `bash-completion` discovery:

```bash
mkdir -p ~/.local/share/bash-completion/completions
yoo completions bash > ~/.local/share/bash-completion/completions/yoo
```

The `bash-completion` package must be installed and loaded by your shell. Start a new Bash session after writing the file.

## Zsh

Create a user completion directory and generate the `_yoo` function:

```zsh
mkdir -p ~/.zfunc
yoo completions zsh > ~/.zfunc/_yoo
```

Add these lines to `~/.zshrc` before any existing `compinit` call:

```zsh
fpath=(~/.zfunc $fpath)
autoload -Uz compinit
compinit
```

Restart Zsh or run `exec zsh`.

## Fish

```fish
mkdir -p ~/.config/fish/completions
yoo completions fish > ~/.config/fish/completions/yoo.fish
```

Fish discovers the file automatically in new sessions. Reload immediately with:

```fish
source ~/.config/fish/completions/yoo.fish
```

## PowerShell

Load completions for the current session:

```powershell
yoo completions powershell | Out-String | Invoke-Expression
```

For persistent completions, create the profile if needed and add the same command:

```powershell
if (!(Test-Path -LiteralPath $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force | Out-Null
}

Add-Content -LiteralPath $PROFILE `
    -Value 'yoo completions powershell | Out-String | Invoke-Expression'
```

Restart PowerShell or reload the profile:

```powershell
. $PROFILE
```

## Troubleshooting

Confirm that the generator works:

```bash
yoo completions bash | head
```

If completions describe an older command set, check `yoo --version` and regenerate the installed completion file. If `yoo` is not found while the shell starts, ensure its installation directory is added to `PATH` before loading completions.
