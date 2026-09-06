# yoo

**What the hell is going on with this project?**

A fast, local-first CLI for understanding your project and development environment. `yoo` reads project files, Git state, installed tooling, and your local configuration, then reports what matters in the terminal.

[![CI](https://github.com/nihitdev/yo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/nihitdev/yo-cli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/nihitdev/yo-cli?sort=semver)](https://github.com/nihitdev/yo-cli/releases)
[![crates.io](https://img.shields.io/crates/v/yoo.svg)](https://crates.io/crates/yoo)
[![npm](https://img.shields.io/npm/v/%40nihit_dev%2Fyoo)](https://www.npmjs.com/package/@nihit_dev/yoo)
[![License](https://img.shields.io/github/license/nihitdev/yo-cli)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-8b8b9a)](docs/installation.md)

## Why yoo?

- **Local-first:** project data stays on your machine.
- **Fast:** one small Rust executable with no daemon or account.
- **Useful context:** project identity, Git status, toolchain health, and session reminders in one place.
- **Terminal-native:** readable output by default, stable JSON when another tool needs it.
- **Cross-platform:** Linux, macOS, and Windows, with package channels for each.
- **No telemetry or AI requirement.**

## Quick start

```bash
cargo install yoo
cd path/to/a/project
yoo --fast
```

The verified installer is convenient when Rust is not already installed:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://yo-cli.vercel.app/yo-setup | sh
```

## Installation

Choose one method. The full platform notes and verification steps are in [docs/installation.md](docs/installation.md).

### Linux

| Channel | Install | Status |
| --- | --- | --- |
| APT (Debian/Ubuntu) | `curl -fsSL https://yo-cli.vercel.app/apt/setup.sh \| sudo sh` then `sudo apt install yoo` | Self-hosted signed repository |
| DNF (Fedora/RHEL) | `sudo dnf config-manager addrepo --from-repofile=https://yo-cli.vercel.app/rpm/yoo.repo` then `sudo dnf install yoo` | Self-hosted signed repository |
| Alpine | Add `https://yo-cli.vercel.app/alpine` to `/etc/apk/repositories`, then `sudo apk add yoo` | Self-hosted APK repository |
| openSUSE Tumbleweed | `sudo zypper ar -f https://yo-cli.vercel.app/opensuse yoo && sudo zypper refresh && sudo zypper install yoo` | Self-hosted signed repository |
| Arch / AUR | `yay -S yoo-bin` | Community package |
| Nix | `nix run github:nihitdev/yo-cli` | GitHub flake |
| Flatpak | `flatpak remote-add --user yoo https://yo-cli.vercel.app/flatpak/yoo.flatpakrepo && flatpak install --user yoo io.github.nihitdev.yoo` | Self-hosted repository |
| Cargo Binstall | `cargo binstall yoo` | Prebuilt GitHub Release binary |

`.deb`, `.rpm`, `.apk`, `.xbps`, and `.snap` files are also attached to each GitHub Release for direct installation. Void packages are built and tested in CI; there is no hosted XBPS repository, and Snap Store publication is disabled.

### macOS

```bash
brew tap nihitdev/tap
brew install yoo
```

Or use `cargo install yoo`, `cargo binstall yoo`, or the verified installer.

### Windows

```powershell
winget install --id Nihitdev.yoo --exact
choco install yoo
```

Scoop metadata is generated with releases, but the community bucket is not currently published. Chocolatey and WinGet community review can delay availability of a new version.

### JavaScript

The npm package downloads the matching native release binary during installation. pnpm and Bun use the same package:

```bash
npm install -g @nihit_dev/yoo
pnpm add -g @nihit_dev/yoo
bun add -g @nihit_dev/yoo
```

Snap builds are attached to GitHub Releases. Snap Store publishing is currently disabled; install a downloaded artifact with `snap install --dangerous` when needed.

## Commands

Run `yoo` inside a project, or pass a path where the command supports it.

| Command | Purpose |
| --- | --- |
| `yoo` | Session summary with project and Git context |
| `yoo doctor` | Check local tools, project detection, and configuration |
| `yoo project` | Project metadata, source counts, and Git details |
| `yoo fetch` | Project plus development environment report |
| `yoo status` | Alias for the project status view |
| `yoo session [MINUTES]` | Start a local coding-session timer |
| `yoo tip` / `yoo tips` | Show a configured reminder or tip pack |
| `yoo edit` | Open the current project in the configured editor |
| `yoo init` / `yoo config` | Create or inspect local configuration |
| `yoo completions SHELL` | Generate Bash, Zsh, Fish, or PowerShell completions |

Useful output flags include `--fast`, `--plain`, `--no-art`, `--theme NAME`, and `--json` for `project` and `fetch`.

```bash
yoo doctor
yoo project --json
yoo fetch --plain
yoo session 25
```

The default report includes the installed version and current project context:

```text
Version:         1.0.0
Project:         yoo
Git:             main · clean
```

## Themes

`yoo` includes **neon**, **ocean**, **mono**, **dracula**, **tokyo-night**, **gruvbox**, **nord**, **rose-pine**, and **catppuccin**. Set one in `~/.config/yoo/config.toml` or pass `--theme NAME`.

## Privacy

Normal operation is local. `yoo` does not require an account, daemon, cloud service, telemetry, or network connection. It reads the current project and development environment and prints the result; project data is not uploaded.

## Building from source

Rust 1.85 or newer and Cargo are required for the CLI:

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli
cargo build --release --locked
cargo test --locked
```

Node.js is only needed when working on the npm wrapper or the website in `site/`.

Release maintainers should follow [docs/releasing.md](docs/releasing.md). Packaging notes live in [packaging/README.md](packaging/README.md).

## Contributing

Issues and pull requests are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before making a change. Security reports belong in [SECURITY.md](SECURITY.md).

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
