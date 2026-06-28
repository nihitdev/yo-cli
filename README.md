# yoo

A tiny Rust CLI that starts your coding session with good vibes — now with project checks, local focus timers, themes, and YAML-powered tip packs.

```bash
cargo install yoo
```

```bash
yoo --fast --name Nihit
```

```text
yoo — developer session starter
What are we shipping today, Nihit? 🚀
💧 Hydration: Water first. Then infinite debugging power.

────────────────────────────────────────
📁 Project: yo-cli
🌿 Git branch: main
✏️ Working tree: clean
────────────────────────────────────────
💡 Tip: Commit small, meaningful changes before starting the next feature.
Go build something fun. 👋
```

## Commands

```bash
yoo                         # Start a session greeting
yoo doctor                  # Check Rust, Cargo, Git, config, and the current project
yoo session                 # Start the configured local coding timer
yoo session 45              # Start a 45-minute local coding timer
yoo tip                     # Print one tip from the configured pack
yoo tip rust                # Print one tip from the Rust pack
yoo tips                    # List built-in and local community tip packs
yoo init                    # Create config.yaml and a sample community pack
yoo config                  # Print the config file location
yoo --fast --theme nord     # Skip animation and override theme once
```

`yoo session` is fully local: it does not connect to a server, create an account, or collect session data.

## YAML configuration

Run this once:

```bash
yoo init
```

`yoo` writes `config.yaml` to the normal config folder for your OS:

| OS | Path |
| --- | --- |
| Windows | `%APPDATA%\\yoo\\config.yaml` |
| Linux | `~/.config/yoo/config.yaml` |
| macOS | `~/Library/Application Support/yoo/config.yaml` |

Example:

```yaml
version: 1

profile:
  name: Nihit

appearance:
  theme: tokyo-night
  ascii: true
  colors: true
  typing_speed_ms: 12

git:
  show_branch: true
  show_status: true

tips:
  enabled: true
  pack: rust

hydration:
  enabled: true

session:
  default_minutes: 25
  show_complete_message: true
```

## Themes

`neon`, `ocean`, `mono`, `dracula`, `tokyo-night`, `gruvbox`, `nord`, `rose-pine`, and `catppuccin`.

```bash
yoo --theme dracula
yoo --theme tokyo-night
yoo --theme catppuccin
```

## Tip packs

Built-in packs:

- `general`
- `rust`
- `git`
- `linux`

Run `yoo tips` to see all packs, including local community packs.

Create your own YAML pack in the directory printed by:

```bash
yoo tips
```

Example:

```yaml
name: web
description: Useful web-development reminders.
tips:
  - Test loading, error, and empty states.
  - Check the browser console before guessing.
```

A local pack with the same name as a built-in pack replaces that built-in pack on your machine.

## Development

```bash
cargo fmt
cargo test
cargo clippy -- -D warnings
cargo run -- doctor
```

## License

`yoo` is licensed under the GNU General Public License v3.0 or later (`GPL-3.0-or-later`). See [LICENSE](LICENSE).
