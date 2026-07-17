# yoo

<p align="center">
  <strong>A tiny developer companion for better coding sessions.</strong>
</p>

<p align="center">
  Start a terminal session with project details, environment checks, source statistics, Git status, tip packs, themes, and a local focus timer.
</p>

<p align="center">
  <a href="https://crates.io/crates/yoo">
    <img src="https://img.shields.io/crates/v/yoo?style=for-the-badge&logo=rust&label=crates.io" alt="Crates.io version">
  </a>
  <a href="https://crates.io/crates/yoo">
    <img src="https://img.shields.io/crates/d/yoo?style=for-the-badge&label=crate%20downloads" alt="Crates.io downloads">
  </a>
  <a href="https://www.npmjs.com/package/@nihitde_v/yoo">
    <img src="https://img.shields.io/npm/v/@nihitde_v/yoo?style=for-the-badge&logo=npm&label=npm" alt="npm version">
  </a>
  <a href="https://github.com/nihitdev/yo-cli/releases/latest">
    <img src="https://img.shields.io/github/v/release/nihitdev/yo-cli?style=for-the-badge&logo=github" alt="Latest GitHub release">
  </a>
</p>

<p align="center">
  <a href="https://github.com/nihitdev/yo-cli/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/nihitdev/yo-cli/ci.yml?branch=main&style=flat-square&logo=githubactions&label=CI" alt="CI status">
  </a>
  <a href="https://github.com/nihitdev/yo-cli/releases">
    <img src="https://img.shields.io/github/downloads/nihitdev/yo-cli/total?style=flat-square&logo=github" alt="GitHub downloads">
  </a>
  <a href="https://github.com/nihitdev/yo-cli">
    <img src="https://img.shields.io/github/repo-size/nihitdev/yo-cli?style=flat-square" alt="Repository size">
  </a>
  <img src="https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust" alt="Rust 2024 edition">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey?style=flat-square" alt="Supported platforms">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue?style=flat-square" alt="GPL-3.0-or-later license">
  </a>
</p>

## What is yoo?

`yoo` is a fast Rust CLI that makes opening a terminal feel a little better.

It gives you a friendly developer-session greeting, shows project and Git state, checks your Rust setup, fetches environment information, analyses the project in the current directory, offers practical tips, and includes a local focus timer.

**Private by default:** no telemetry, no daemon, no AI service, and no project data sent anywhere.

```text
Terminal open. Brain online. Let's go. ⚡

📁 Project: yo-cli
🌿 Git branch: main
✏️ Working tree: clean

💡 Tip: Write the test that would have caught your last bug.
```

## Quick start

```bash
cargo binstall yoo
yoo
yoo doctor
yoo project
```

Use `yoo --fast` when you want the greeting without the typewriter animation.

## Features

- 📊 Counts source files and lines while skipping dependencies, caches, and build output
- 🚀 Friendly developer session starter
- 🩺 `yoo doctor` for Rust, Cargo, Git, config, and project checks
- ⚡ `yoo fetch` for developer-environment and project detection
- 📦 `yoo project` for project metadata, source stats, Git details, and project-file checks
- 📄 JSON output with `yoo fetch --json` and `yoo project --json`
- ⏱️ Local coding-session timer with `yoo session`
- 📝 YAML configuration
- 💡 Built-in and community YAML tip packs
- 🌿 Current Git branch and working-tree status
- 🎨 Nine terminal themes
- 🦀 Written in Rust
- ✅ Unit tests, formatting checks, Clippy, and GitHub Actions CI

## Why use it?

- Start a coding session with the project name, Git branch, working-tree state, and one useful reminder.
- Check whether Rust, Cargo, Git, Rustfmt, Clippy, config, and repository basics are available.
- Get a quick project report without opening an IDE.
- Feed `yoo fetch --json` or `yoo project --json` into scripts.
- Keep personal and team tips in simple YAML files.

## Screenshots

### Start a coding session

```bash
yoo --fast --name YourName
```

<p align="center">
  <img src="docs/images/hero.png" alt="yoo welcome screen" width="780">
</p>

### Check your setup

```bash
yoo doctor
```

<p align="center">
  <img src="docs/images/doctor.png" alt="yoo doctor output with Rust and Git checks" width="780">
</p>

### Fetch your developer environment

```bash
yoo fetch
```

<p align="center">
  <img src="docs/images/fetch.png" alt="yoo fetch showing developer environment and project status" width="780">
</p>

### Inspect the current project

```bash
yoo project
```

```text
yoo project — project overview

📦 Name:            yoo
🔧 Language:        Rust
📦 Package manager: Cargo
📄 Manifest:        Cargo.toml
🏷 Version:         0.6.4
🦀 Edition:         2024
⚖ License:          GPL-3.0-or-later

📁 Source files:    13
📏 Source lines:    2,936
🌿 Git branch:      main
✏️ Working tree:    clean
📜 Commits:         36
🏷 Latest tag:      v0.6.4
```

## 📦 Installation

### Cargo Binstall (Fastest)

Download a prebuilt binary from GitHub Releases instead of compiling from source:

```bash
cargo binstall yoo
```

Install Cargo Binstall first when needed:

```bash
cargo install cargo-binstall
```

### Cargo

```bash
cargo install yoo
```

### npm

```bash
npm install -g @nihitde_v/yoo
```

### pnpm

```bash
pnpm add -g @nihitde_v/yoo
```

### Bun

```bash
bun add -g @nihitde_v/yoo
```

### Scoop (Windows)

```powershell
scoop bucket add nihitdev https://github.com/nihitdev/scoop-bucket
scoop install yoo
```

### Chocolatey (Windows)

```powershell
choco install yoo
```

> **Note:** The Chocolatey package is currently awaiting moderation. Once approved, the command above will work.

### WinGet (Windows)

```powershell
winget install Nihitdev.Yoo
```

> **Note:** The WinGet package is currently under review by Microsoft.

### Arch Linux (AUR)

```bash
yay -S yoo-bin
```

### Build from Source

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli
cargo build --release
cargo install --path .
```

Start your first developer session:

```bash
yoo
```

## Commands

| Command | Purpose |
| :-- | :-- |
| `yoo` | Start the default developer-session greeting |
| `yoo --fast` | Start the greeting without the typewriter delay |
| `yoo doctor` | Check local tooling, config, and repository basics |
| `yoo fetch` | Show OS, shell, editor, Rust/Cargo/Git, project, and Git state |
| `yoo fetch --json` | Print the fetch report as JSON |
| `yoo status` | Alias for `yoo fetch` |
| `yoo project` | Show project metadata, source stats, Git details, and project-file checks |
| `yoo project --json` | Print the project report as JSON |
| `yoo session` | Start the configured focus timer |
| `yoo session 25` | Start a 25-minute focus timer |
| `yoo tip rust` | Print one tip from the Rust tip pack |
| `yoo tips` | List built-in and local tip packs |
| `yoo init` | Create the default config and sample community tip pack |
| `yoo config` | Print the active config path |
| `yoo version` | Print the installed version |
| `yoo help` | Print command help |

## Useful options

```bash
yoo --fast
yoo --name YourName
yoo --theme tokyo-night
yoo --plain
yoo --no-art
yoo project --plain
yoo fetch --json
yoo project --json
```

`--json` is intentionally decoration-free and cannot be combined with display options such as `--plain`, `--no-art`, or `--theme`.

## Themes

```text
neon
ocean
mono
dracula
tokyo-night
gruvbox
nord
rose-pine
catppuccin
```

Example:

```bash
yoo --fast --theme tokyo-night
```

## Project detection

`yoo project` and `yoo fetch` detect these project markers:

| Project type | Marker | Package-manager detection |
| :-- | :-- | :-- |
| Rust | `Cargo.toml` | Cargo |
| Node.js | `package.json` | npm, pnpm, Yarn, or Bun |
| Python | `pyproject.toml` | pip, uv, Poetry, or Pipenv |
| Go | `go.mod` | Go modules |
| Java | `pom.xml` or Gradle files | Maven or Gradle |
| .NET | `.sln` or `.csproj` | .NET SDK |

`yoo project` counts source files and lines while skipping generated or heavy folders such as `.git`, `target`, `node_modules`, `dist`, `build`, `.next`, `.venv`, and `vendor`.

## JSON output

Use JSON output when scripting or feeding project information into another tool:

```bash
yoo fetch --json
yoo project --json
```

Example fields include:

```json
{
  "yoo_version": "0.6.4",
  "project": {
    "name": "yoo",
    "language": "Rust",
    "version": "0.6.4"
  },
  "git": {
    "branch": "main",
    "changed_files": 0
  }
}
```

The exact report includes more fields, but it stays focused on local environment, project, source, and Git data.

## Privacy

`yoo` runs locally. It does not use AI services, collect telemetry, run a background service, or send project data anywhere. Environment and project information stays on your machine and is written only to the requested terminal or JSON output.

## Configuration

Create the default YAML configuration and a sample community tip pack:

```bash
yoo init
```

Config locations:

```text
Windows: %APPDATA%\yoo\config.yaml
Linux:   ~/.config/yoo/config.yaml
macOS:   ~/Library/Application Support/yoo/config.yaml
```

Print the active config path:

```bash
yoo config
```

Default config:

```yaml
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
```

## Tip packs

`yoo` ships with these built-in tip packs:

```text
general
git
linux
rust
```

Get one random Rust tip:

```bash
yoo tip rust
```

Community packs are YAML files stored here:

```text
Windows: %APPDATA%\yoo\tips
Linux:   ~/.config/yoo/tips
macOS:   ~/Library/Application Support/yoo/tips
```

Example local tip pack:

```yaml
name: team
description: Team workflow reminders.
tips:
  - Keep pull requests small enough to review carefully.
  - Write down the command that fixed the problem.
```

Then run:

```bash
yoo tip team
```

## Troubleshooting

| Problem | What to try |
| :-- | :-- |
| `yoo doctor` says config is missing | Run `yoo init`; defaults are still used until then |
| No colours appear | Check whether output is redirected, or use a terminal that supports ANSI colours |
| Git branch is missing | Run the command inside a Git repository and make sure `git` is in `PATH` |
| `cargo install yoo` takes too long | Use `cargo binstall yoo` to download a prebuilt binary |
| `cargo install yoo` fails | Update Rust with `rustup update`, then retry |
| JSON command rejects display flags | Remove `--plain`, `--no-art`, or `--theme` when using `--json` |

## Development

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli

cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo build --release

cargo run -- doctor
cargo run -- fetch
cargo run -- project
cargo run -- project --json
```

Release checks used by this repo:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked -- -D warnings
cargo build --release --locked
```

## Roadmap

- [x] Developer session greeting
- [x] Git branch and working-tree summary
- [x] YAML configuration
- [x] Themes
- [x] `yoo doctor`
- [x] Local coding-session timer
- [x] Community YAML tip packs
- [x] `yoo fetch` developer environment and project status
- [x] `yoo project` project overview and source stats
- [x] JSON output for `yoo fetch` and `yoo project`
- [x] AUR package
- [x] Automated cross-platform GitHub releases
- [x] Cargo Binstall support
- [ ] More tip packs from contributors
- [ ] Shell completion support
- [ ] Better terminal accessibility options
## Package ecosystem

| Platform | Command | Status |
| :-- | :-- | :--: |
| Cargo Binstall | `cargo binstall yoo` | ✅ |
| crates.io | `cargo install yoo` | ✅ |
| npm | `npm install -g @nihitde_v/yoo` | ✅ |
| pnpm | `pnpm add -g @nihitde_v/yoo` | ✅ |
| Bun | `bun add -g @nihitde_v/yoo` | ✅ |
| Scoop | `scoop install yoo` | ✅ |
| Arch Linux (AUR) | `yay -S yoo-bin` | ✅ |
| Chocolatey | `choco install yoo` | ⏳ Review |
| WinGet | `winget install Nihitdev.Yoo` | ⏳ Review |

## Contributing

Contributions, ideas, tip packs, and bug reports are welcome.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

`yoo` is licensed under the GNU General Public License v3.0 or later.

See [LICENSE](LICENSE) for details.

---

<p align="center">Built with ❤️ and Rust by <a href="https://github.com/nihitdev">@nihitdev</a></p>

