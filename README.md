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

Commands inspect the current directory. Use `cd path/to/project` first; positional project paths are not supported.

| Command | Purpose |
| --- | --- |
| `yoo` | Session summary with project and Git context |
| `yoo doctor` | Check local tools, project detection, and configuration |
| `yoo project` | Project metadata, source counts, and Git details |
| `yoo snapshot` | Save a local snapshot of the current project |
| `yoo snapshot list` | List saved snapshots in reverse chronological order |
| `yoo snapshot compare` | Compare the two newest local snapshots |
| `yoo fetch` | Project plus development environment report |
| `yoo status` | Alias for `yoo fetch` (environment and project report) |
| `yoo session [MINUTES]` | Start a local coding-session timer |
| `yoo tip` / `yoo tips` | Show a configured reminder or tip pack |
| `yoo edit` | Open the current project in the configured editor |
| `yoo init` / `yoo config` | Create configuration and a sample tip pack / print the configuration file path |
| `yoo completions SHELL` | Generate Bash, Zsh, Fish, or PowerShell completions |

Useful output flags include `--fast`, `--plain`, `--no-art`, `--theme NAME`, and `--json` for `project` and `fetch`.

```bash
yoo doctor
yoo project --json
yoo fetch --plain
yoo snapshot
# make changes, then capture another point in time
yoo snapshot
yoo snapshot list
yoo snapshot compare
yoo session 25
```

Snapshots are saved as JSON files in `.yoo/snapshots/` inside the current
project. They stay on your machine and are excluded from Git by the default
`.gitignore` entry. `snapshot compare` compares the two newest saved snapshots;
save another snapshot after making changes to track progress over time.

Check the installed CLI version with `yoo --version`:

```text
yoo 1.1.2
```

`project --json` and `fetch --json` include `yoo_version`. Their `project.version`
field is the inspected project's version, when available. `--json` cannot be
combined with `--plain`, `--no-art`, or `--theme`.

Git inspection distinguishes clean and dirty repositories, repositories before
their first commit, and non-Git directories. Git failures and timeouts are
reported on stderr with exit status 1 (including in JSON mode), rather than
being reported as a clean tree. Exit status 2 indicates invalid CLI arguments.

`yoo doctor` prints all checks and exits 1 if any check fails; warnings alone
exit 0. The tool checks currently require Rust, Cargo, Git, Rustfmt, and Clippy.
These are development health checks, not dependencies needed to launch `yoo`.

Source totals include all supported source extensions across languages, respect
nested `.gitignore` rules, `.git/info/exclude`, and global Git excludes, and skip
symlinks and common generated/vendor directories (`target`, `node_modules`,
`dist`, `build`, `.next`, `.venv`, `venv`, `vendor`, `__pycache__`, `.git`). Counts
also honor `.ignore` files. Files that cannot be read are skipped.

## Configuration

Run `yoo init` to create the default configuration without overwriting an
existing file. Run `yoo config` to print its exact location, then edit that file.
See [examples/config.toml](examples/config.toml) for supported settings.

Default locations:

- Linux: `$XDG_CONFIG_HOME/yoo/config.toml`, or `~/.config/yoo/config.toml`.
- macOS: `~/Library/Application Support/yoo/config.toml`.
- Windows: `%USERPROFILE%\.config\yoo\config.toml`.

The editor setting accepts a single executable name or path, not a shell command
with arguments. `yoo edit --editor` overrides it for one invocation; otherwise
`VISUAL`, `EDITOR`, and automatic detection are used when it is empty.

## Troubleshooting

Run `yoo doctor` for tool, configuration, project, and Git diagnostics. Use
`yoo config` to locate invalid configuration. If Git inspection fails, check
`git status` in the same directory; commands have a five-second execution timeout.
For installation and PATH issues, see [the installation guide](docs/installation.md).

## Themes

`yoo` includes **neon**, **ocean**, **mono**, **dracula**, **tokyo-night**, **gruvbox**, **nord**, **rose-pine**, and **catppuccin**. Set one in the file reported by `yoo config`, or pass `--theme NAME`.

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
