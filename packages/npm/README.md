# yoo

**Open a repo. Know where to start.**

Project details, Git status, development tools, snapshots, and a focus timer in
your terminal. Built in Rust; this npm package installs a prebuilt binary.

![yoo running in a terminal](https://raw.githubusercontent.com/nihitdev/yo-cli/main/docs/images/hero.png)

## Install

Requires Node.js 18 or newer. Choose one package manager:

```sh
npm install -g @nihit_dev/yoo
```

```sh
pnpm add -g @nihit_dev/yoo
```

```sh
bun add -g @nihit_dev/yoo
```

Prebuilt binaries are available for **Linux x64, Windows x64, and Apple Silicon
macOS**. For other targets, build from source with Rust 1.85 or newer:

```sh
cargo install yoo --locked
```

## Try it in your project directory

Get oriented:

```sh
yoo --fast
```

Inspect the project:

```sh
yoo project
```

Check the project's development tools:

```sh
yoo doctor
```

Save a project snapshot:

```sh
yoo snapshot
```

Compare it after making changes:

```sh
yoo snapshot compare --current
```

Start a focus timer:

```sh
yoo session 25
```

Get machine-readable output:

```sh
yoo project --json
```

[All commands](https://github.com/nihitdev/yo-cli/blob/main/docs/commands.md) ·
[Configuration](https://github.com/nihitdev/yo-cli/blob/main/docs/configuration.md) ·
[Shell completions](https://github.com/nihitdev/yo-cli/blob/main/docs/completions.md)

## How installation works

The package downloads the binary from the GitHub release matching its package
version and verifies it against that release's `SHA256SUMS`. Installation uses
HTTPS, bounded redirects, download deadlines, and atomic replacement. Failed
downloads leave an existing executable untouched. No additional npm dependencies
are required. The CLI itself needs no account, daemon, or network service.

For local builds, `YOO_BINARY_PATH` can point to a nonempty native executable.
This explicit override copies that file without downloading or verifying release
hashes. Maintainers can run the installer regression suite with `npm test`.

## Troubleshooting

Check the installed version:

```sh
yoo --version
```

If the binary is missing, reinstall the npm package with lifecycle scripts
allowed. Corporate proxies or blocked GitHub downloads can also prevent
installation; see the
[installation guide](https://github.com/nihitdev/yo-cli/blob/main/docs/installation.md).

[Source and screenshots](https://github.com/nihitdev/yo-cli) ·
[Report an issue](https://github.com/nihitdev/yo-cli/issues/new/choose) ·
[GPL-3.0-or-later](https://github.com/nihitdev/yo-cli/blob/main/LICENSE)
