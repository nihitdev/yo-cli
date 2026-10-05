# Configuration

[← Back to yoo](../README.md)

Run `yoo init` to create the default configuration without overwriting an
existing file. Run `yoo config` to print its exact location, then edit that file.
See [examples/config.toml](../examples/config.toml) for supported settings.

Default locations:

- Linux: `$XDG_CONFIG_HOME/yoo/config.toml`, or `~/.config/yoo/config.toml`.
- macOS: `~/Library/Application Support/yoo/config.toml`.
- Windows: `%USERPROFILE%\.config\yoo\config.toml`.

The editor setting accepts a single executable name or path, not a shell command
with arguments. `yoo edit --editor` overrides it for one invocation; otherwise
`VISUAL`, `EDITOR`, and automatic detection are used when it is empty.

## Themes

`yoo` includes **neon**, **ocean**, **mono**, **dracula**, **tokyo-night**, **gruvbox**, **nord**, **rose-pine**, and **catppuccin**. Set one in the file reported by `yoo config`, or pass `--theme NAME`.
