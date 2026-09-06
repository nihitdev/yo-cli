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

## Homebrew

Supports Apple Silicon macOS and Linux x86_64. The public
`nihitdev/homebrew-tap` repository was not found during implementation.
Once its owner creates it and installs the generated formula:

```bash
brew tap nihitdev/tap
brew install yoo
```

Until then, download `yoo-packaging.tar.gz` from the desired GitHub Release,
verify its checksum, and install its formula into a local tap:

```bash
tar -xzf yoo-packaging.tar.gz
brew tap-new local/yoo
cp homebrew/yoo.rb "$(brew --repository local/yoo)/Formula/yoo.rb"
brew install local/yoo/yoo
brew test local/yoo/yoo
```

The checked-in `packaging/homebrew/yoo.rb` can also be copied to that tap.
The formula verifies SHA-256. Linux release binaries are built on Ubuntu 22.04;
older glibc systems should use Cargo or Nix.

## Arch Linux / AUR

The binary package is prepared for publication as `yoo-bin`:

```bash
yay -S yoo-bin
```

This requires the maintainer to publish the AUR repository first.
You can build it now from this checkout:

```bash
cd packaging/aur
makepkg -si
```

It verifies the release archive, installs `/usr/bin/yoo`, documentation,
license and Bash/Zsh/Fish completions. Git is an optional dependency.
Binary packaging reuses the release binary and avoids a Rust compilation.

## Nix

The flake builds from Rust source with `Cargo.lock` and pinned Nixpkgs.
Linux x86_64/ARM64 and Apple Silicon macOS are exposed:

```bash
nix run github:nihitdev/yo-cli
nix profile install github:nihitdev/yo-cli
# From a checkout:
nix build
nix flake check
```

Enable Nix's `nix-command` and `flakes` experimental features if needed.
Host Git, development tools and editors are detected from PATH.

## Debian / Ubuntu

Download `yoo_<version>-1_amd64.deb` and `SHA256SUMS` from the same release:

```bash
sha256sum --check --ignore-missing SHA256SUMS
sudo dpkg -i yoo_*.deb
# If dpkg reports missing dependencies:
sudo apt-get -f install
```

Build on Debian/Ubuntu with `dpkg-dev` installed so library dependencies can be
detected (other host distributions lack Debian package ownership information):

```bash
cargo install cargo-deb --locked
cargo deb
```

Output is in `target/debian/`. Runtime library dependencies are generated from
the binary. Release packages target x86_64, built on Ubuntu 22.04; native builds
on other Linux architectures use the local toolchain.

## RPM

Download `yoo-<version>-1.x86_64.rpm` and verify the release's `SHA256SUMS`:

```bash
sha256sum --check --ignore-missing SHA256SUMS
sudo rpm -i yoo-*.rpm
```

Or use `sudo dnf install ./yoo-*.rpm` to resolve dependencies. Build locally:

```bash
cargo install cargo-generate-rpm --locked
cargo build --release --locked
cargo generate-rpm
```

Output is in `target/generate-rpm/`. Runtime library requirements are automatically
detected. These are unsigned standalone packages, not an apt/yum repository.
The release baseline is Ubuntu 22.04's glibc; older distributions may need a
native source build.

## Snap

After the maintainer registers the name and publishes a stable channel:

```bash
sudo snap install yoo
```

For a downloaded release artifact (locally built snaps are unsigned):

```bash
sudo snap install --dangerous ./yoo_*.snap
snap run yoo --version
```

The snap uses strict confinement. `home` provides access to non-hidden home
projects; `removable-media` is optional and must be explicitly connected to
inspect projects under mounted media:

```bash
sudo snap connect yoo:removable-media
```

There is no network interface. Hidden home paths, arbitrary system directories,
host toolchains, and host editors are unavailable. Git is bundled for repository
inspection, but user-level Git configuration outside the sandbox may be absent.
Configuration lives in the snap's own home. `doctor`, environment reporting and
`edit` do not represent the full host environment. Choose a native package for
complete functionality. No classic-confinement approval is needed.

Build with `snapcraft` from the repository root. The version comes from
`Cargo.toml` through `adopt-info`; the manifest supports amd64 and arm64, with
amd64 built automatically by CI.

## Flatpak

Flatpak is an imperfect fit for this terminal CLI. The manifest grants broad
**read-only host filesystem access** to inspect local projects. Flatpak still
masks some system paths and does not expose host executable toolchains through
the sandbox PATH. Git/tool detection and launching host editors are limited;
configuration is stored under the app's sandbox directory. It does not grant
host execution, network access or general host write access.

There is no Flathub listing. Install the release bundle locally:

```bash
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install --user flathub org.freedesktop.Platform//25.08
flatpak install --user ./yoo-*.flatpak
flatpak run io.github.nihitdev.yoo --version
flatpak run --cwd="$PWD" io.github.nihitdev.yoo project
```

For source builds, install `flatpak-builder`, the matching Freedesktop SDK and
Rust SDK extension, then:

```bash
flatpak install --user flathub org.freedesktop.Sdk//25.08 org.freedesktop.Sdk.Extension.rust-stable//25.08
bash scripts/flatpak-source.sh
flatpak-builder --user --disable-rofiles-fuse --repo=target/flatpak-repo target/flatpak-build packaging/flatpak/io.github.nihitdev.yoo.yml
flatpak build-bundle target/flatpak-repo target/yoo.flatpak io.github.nihitdev.yoo
```

The preparation step vendors locked Cargo dependencies; compilation is offline
inside Flatpak. No network permission is granted to the installed application.

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
