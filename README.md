# yoo

> A tiny Rust CLI that starts your coding session with good vibes, a useful tip, and a quick Git check.

```text
██╗   ██╗ ██████╗  ██████╗
╚██╗ ██╔╝██╔═══██╗██╔═══██╗
 ╚████╔╝ ██║   ██║██║   ██║
  ╚██╔╝  ██║   ██║██║   ██║
   ██║   ╚██████╔╝╚██████╔╝
   ╚═╝    ╚═════╝  ╚═════╝

yoo — developer session starter
YOOOOO, Nihit! 🔥
💧 Hydration: Drink some water before the bug drinks your sanity.

────────────────────────────────────────
📁 Project: yoo
🌿 Git branch: main
✏️ Working tree: clean
────────────────────────────────────────
💡 Tip: Commit small, meaningful changes before starting the next feature.
Go build something fun. 👋
```

## Why yoo?

`yoo` is deliberately small. It gives you a fun terminal welcome, reminds you to hydrate, shows the current project and Git state, and gives a short developer tip—without needing an account, network access, or third-party runtime dependencies.

## Install

```bash
cargo install yoo
```

For local development:

```bash
git clone https://github.com/nihitdev/yo-cli
cd yo-cli
cargo run -- --fast
```

## Commands

```text
yoo                         Run with your saved settings
yoo init                    Create a starter config file
yoo config                  Print the config-file location
yoo --fast                  Skip typewriter animation
yoo --no-art                Hide the ASCII logo for one run
yoo --plain                 Disable terminal colours
yoo --name Nihit            Use a name for one run
yoo --theme ocean           Use neon, ocean, or mono for one run
yoo --help                  Show all options
yoo --version               Show the installed version
```

## Configuration

Create a config file:

```bash
yoo init
yoo config
```

Then edit the generated `config.toml`:

```toml
name = "Nihit"
theme = "neon"
typing_speed_ms = 12
show_art = true
show_git = true
```

## Development

```bash
cargo fmt --all -- --check
cargo test
cargo run -- --fast --name Nihit
```

## Project structure

```text
.
├── .github/workflows/ci.yml    # Builds and tests on GitHub Actions
├── examples/config.toml        # Example user configuration
├── src/
│   ├── app.rs                  # Application flow
│   ├── args.rs                 # Command-line argument parser
│   ├── config.rs               # Tiny config loader and writer
│   ├── content.rs              # Greetings, tips, reminders
│   ├── git.rs                  # Safe Git branch/status lookup
│   ├── main.rs                 # Binary entry point
│   └── ui.rs                   # ANSI terminal UI
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE
└── README.md
```

## Roadmap

- [x] Custom names, themes, config, and Git status
- [x] Unit tests and cross-platform CI
- [ ] `yoo doctor` to check Git/Rust/project setup
- [ ] Optional local coding-session timer
- [ ] More themes and community tip packs

## Contributing

Found a bug, have a better greeting, or want to add a feature? Read [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).