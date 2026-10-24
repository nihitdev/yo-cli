<p align="center">
  <img src="https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/banner.svg" alt="yoo — know the project you’re in" width="100%" />
</p>

<p align="center">
  <strong>Open a repo. Know where to start.</strong><br />
  Project details, Git status, and development tools in your terminal.
</p>

<p align="center">
  <a href="https://github.com/nihitdev/yo-cli/actions/workflows/ci.yml"><img src="https://github.com/nihitdev/yo-cli/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://crates.io/crates/yoo"><img src="https://img.shields.io/crates/v/yoo?color=cba6f7" alt="crates.io" /></a>
  <a href="https://www.npmjs.com/package/@nihit_dev/yoo"><img src="https://img.shields.io/npm/v/%40nihit_dev%2Fyoo?color=89b4fa" alt="npm" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-a6e3a1" alt="GPL-3.0-or-later" /></a>
</p>

<p align="center">
  <a href="#install">Install</a> ·
  <a href="#try-it">Try it</a> ·
  <a href="#screenshots">Screenshots</a> ·
  <a href="docs/README.md">Documentation</a> ·
  <a href="https://github.com/nihitdev/yo-cli/releases">Releases</a>
</p>

![yoo running in a real terminal](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/hero.png)

`yoo` is a local CLI built in Rust for Linux, macOS, and Windows. See the project
you’re in, check its tools, inspect Git changes, save project snapshots, or start
a focus timer. No account, background service, telemetry, or AI connection is
required to use the tool.

## Install

Choose one installation method. Use GitHub’s copy button in the top-right
corner of each command block.

### Arch Linux (AUR)

```sh
yay -S yoo-bin
```

### Cargo — build from source

Requires Rust 1.85 or newer. Use this on other supported Rust targets too.

```sh
cargo install yoo --locked
```

### Cargo Binstall — prebuilt binary

```sh
cargo binstall yoo
```

### pnpm

```sh
pnpm add -g @nihit_dev/yoo
```

### Bun

```sh
bun add -g @nihit_dev/yoo
```

### Homebrew

```sh
brew tap nihitdev/tap
```

```sh
brew install yoo
```

### Windows (WinGet)

```powershell
winget install --id Nihitdev.yoo --exact
```

### Windows (Scoop)

```powershell
scoop bucket add nihitdev https://github.com/nihitdev/scoop-nihitdev
```

```powershell
scoop install nihitdev/yoo
```

### Nix — run without installing

```sh
nix run github:nihitdev/yo-cli
```

### Verified shell installer

Linux x86-64 and Apple Silicon macOS; installs to `~/.local/bin`.

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://yo-cli.vercel.app/yo-setup | sh
```

### Debian and Ubuntu (APT)

```sh
curl -fsSL https://yo-cli.vercel.app/apt/setup.sh | sudo sh
```

```sh
sudo apt install yoo
```

### npm — prebuilt binary

Requires Node.js 18 or newer. Supports Linux x64, Windows x64, and Apple Silicon
macOS. The installer verifies the release checksum before installing.

```sh
npm install -g @nihit_dev/yoo
```

Other packages, including RPM, APK, XBPS, and Flatpak, are covered in the
[installation guide](docs/installation.md). Download release binaries and
checksums from [GitHub Releases](https://github.com/nihitdev/yo-cli/releases).

## Try it

Open a terminal in your project directory, then run:

```sh
yoo --fast
```

### Understand the project

Show the language, package manager, manifest, source counts, and Git state.

```sh
yoo project
```

### Check your setup

Check the tools that match your project. Optional configuration warnings are
shown separately from failures.

```sh
yoo doctor
```

### Inspect your environment

```sh
yoo fetch
```

### Save a snapshot, then compare

Save a local report before you start working:

```sh
yoo snapshot
```

After making changes, compare the saved report with the current project:

```sh
yoo snapshot compare --current
```

Snapshots stay in `.yoo/snapshots/` and are ignored by Git automatically.

### Start a 25-minute focus session

```sh
yoo session 25
```

### Open the project in your editor

```sh
yoo edit
```

### Get a reminder

```sh
yoo tip rust
```

### Use the output in a script

```sh
yoo project --json
```

[All commands, flags, exit codes, and examples →](docs/commands.md)

## Screenshots

Real captures of the CLI on Linux, including the tools and Git state detected
on the capture machine. Your platform, versions, and warnings will differ.

### Project overview

![Project manifest, source counts, and Git details](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/projects.png)

### Development environment

![Operating system, shell, tool versions, and project context](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/fetch.png)

### Setup checks

![Doctor checks with separate passes, warnings, and failures](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/doctor.png)

<details>
<summary><strong>See the focus timer, tips, and editor</strong></summary>

#### Focus timer

![A running local coding session](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/session.png)

#### Tips

![Available tip packs and a Rust reminder](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/tips.png)

#### Editor

![Opening the project in Neovim with yoo edit](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/edit.png)

</details>

## Make it yours

Create your local configuration:

```sh
yoo init
```

Find its location:

```sh
yoo config
```

Try a theme without changing your configuration:

```sh
yoo --fast --theme catppuccin
```

Nine themes: **neon, ocean, mono, dracula, tokyo-night, gruvbox, nord, rose-pine,
catppuccin**. Configure your editor, timer, ASCII art, colors, and YAML
tip packs in the [configuration guide](docs/configuration.md).

Enable completion for your shell using the
[Bash, Zsh, Fish, and PowerShell instructions](docs/completions.md).

## Supported projects

Rust, Zig, Node.js, Python, Go, Java, Ruby, PHP, Swift, Dart/Flutter, Elixir,
C/C++ with CMake, and .NET.

Detection uses manifests in the current directory. Source counts respect ignore
rules. In mixed projects, the first matching manifest wins.
[Detection markers and precedence →](docs/project-detection.md)

## Troubleshooting

- **Command not found?** Restart your terminal and check that your package manager’s binary directory is on `PATH`.
- **Missing tools or configuration?** Run `yoo doctor`. A missing optional configuration file uses defaults.
- **Unexpected project?** Run from the directory containing its manifest. Project paths are not positional arguments.
- **Need to check the installed version?** Run:

```sh
yoo --version
```

```text
yoo 1.1.3
```

[Installation, updates, and removal](docs/installation.md) ·
[Report a bug](https://github.com/nihitdev/yo-cli/issues/new/choose)

## Build and contribute

```sh
git clone https://github.com/nihitdev/yo-cli.git
```

```sh
cd yo-cli
```

```sh
cargo build --release --locked
```

```sh
bash scripts/check.sh
```

The check script also needs Node.js 24, Bash, and Git. See
[Contributing](CONTRIBUTING.md) for setup and
[Code structure](docs/architecture.md) for module responsibilities.

[Changelog](CHANGELOG.md) · [Release guide](docs/releasing.md) ·
[Security](SECURITY.md) · [GPL-3.0-or-later](LICENSE)
