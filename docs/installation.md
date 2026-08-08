# Installation

`yoo` supports Windows, Linux, and macOS. Choose one installation method and avoid installing the executable through multiple package managers at the same time.

## Verified installer

The installer supports Linux x86-64 and Apple Silicon macOS. It downloads the raw binary from the latest GitHub release, verifies it against the published `SHA256SUMS` file, and installs it to `~/.local/bin/yoo`.

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://yo-cli.vercel.app/yo-setup | sh
```

Ensure the installation directory is on `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add that line to `~/.bashrc`, `~/.zshrc`, or the appropriate shell startup file to make it permanent.

### Install a specific version

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://yo-cli.vercel.app/yo-setup |
  YOO_VERSION=1.0.0 sh
```

### Use a custom destination

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://yo-cli.vercel.app/yo-setup |
  YOO_INSTALL_DIR="$HOME/bin" sh
```

### Inspect the installer before running it

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://yo-cli.vercel.app/yo-setup \
  --output yoo-install.sh
less yoo-install.sh
sh yoo-install.sh
```

## Cargo Binstall

Use a prebuilt release binary without compiling locally:

```bash
cargo binstall yoo
```

Install Cargo Binstall first if necessary:

```bash
cargo install cargo-binstall
```

## Cargo

Build and install from crates.io:

```bash
cargo install yoo
```

## npm, pnpm, or Bun

The JavaScript wrapper downloads the matching prebuilt binary during installation:

```bash
npm install -g @nihitde_v/yoo
```

```bash
pnpm add -g @nihitde_v/yoo
```

```bash
bun add -g @nihitde_v/yoo
```

## Windows package managers

### WinGet

```powershell
winget source update
winget install --id Nihitdev.yoo --exact
```

### Scoop

```powershell
scoop bucket add nihitdev https://github.com/nihitdev/scoop-bucket
scoop install yoo
```

## Build from source

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli
cargo install --path .
```

## Verify the installation

```bash
yoo --version
yoo doctor
```

If the shell cannot find `yoo`, restart the terminal and confirm the installation directory appears in `PATH`.

## Update

Repeat the command for the installation method originally used. Examples:

```bash
cargo install yoo --force
npm update -g @nihitde_v/yoo
```

```powershell
winget upgrade --id Nihitdev.yoo --exact
scoop update yoo
```

## Uninstall

Use the matching package manager:

```bash
cargo uninstall yoo
npm uninstall -g @nihitde_v/yoo
```

For an installer-script installation:

```bash
rm "$HOME/.local/bin/yoo"
```

Removing the executable does not delete the optional configuration file. Run `yoo config` before uninstalling if you need to locate it.
