# Maintaining distribution packages

`Cargo.toml` is the canonical version. `scripts/package-version.sh` validates
stable versions and rejects release tags that disagree with Cargo. Existing raw
binaries, archive names, cargo-binstall aliases, npm wrappers and the installer
remain unchanged.

A tagged release runs the reusable `packaging.yml` workflow, builds Linux .deb,
.rpm, Snap and Flatpak artifacts, and checks Nix on Linux/macOS. Publication
collects all artifacts, generates `yoo-packaging.tar.gz` (Homebrew formula plus
AUR PKGBUILD/.SRCINFO), then generates `SHA256SUMS` over every downloadable file.
Hashes describe the actual build; byte-for-byte reproducibility is not claimed.
The installer continues looking up raw binary names in the same checksum file.

Homebrew and AUR checked-in files are snapshots for the current published
release. Templates are authoritative for future releases. No manual version or
hash editing is required:

```bash
bash scripts/package-metadata.sh /path/to/release-assets
```

The directory must contain both platform tarballs for the Cargo version. Verify
downloaded inputs against that release's `SHA256SUMS` first. Release automation
uses artifacts from the same workflow, writes metadata into `target`, and does
not commit into the source repository. To refresh snapshots later, run the
command above and commit the generated files. `.SRCINFO` is checked against
`makepkg --printsrcinfo` in CI.

Completions are generated from the CLI and checked into `packaging/completions`
so plain `cargo deb` and `cargo generate-rpm` work without preparation. After
changing completions, run `cargo build --release --locked` followed by
`bash scripts/package-completions.sh`. CI checks for drift.

## External setup

- **Homebrew:** create `nihitdev/homebrew-tap` with an initial default branch.
  Optional `HOMEBREW_TAP_TOKEN` needs contents-write access to that repository.
  The release job commits only `Formula/yoo.rb`. Without the secret it skips.
- **AUR:** register an account and SSH key, request/create `yoo-bin`, copy the
  release bundle's `aur/PKGBUILD` and `aur/.SRCINFO` into that AUR Git checkout,
  run `makepkg --verifysource`, `makepkg` and `makepkg --printsrcinfo`, then
  review, commit and push. There is deliberately no automatic AUR publishing.
- **Snap:** register `yoo` in the Snap Store. Optional
  `SNAPCRAFT_STORE_CREDENTIALS` is an exported credential scoped for this snap.
  Release automation uploads to **candidate**, allowing confinement review
  before manually promoting to stable. Without the secret it skips.
- **Flatpak:** bundles require no account. A Flathub submission would require
  separate review and repository setup; the broad read-only filesystem grant
  and terminal-only interface must be evaluated there.
- **Nix:** no registry account is required for GitHub flake URLs.
- **Deb/RPM:** no accounts required for unsigned release artifacts. Signed apt
  or RPM repositories are outside this setup.

See [installation and sandbox limitations](../docs/installation.md). Optional
publication jobs run only after the GitHub Release exists; absent credentials
do not block artifact generation.

## Validation

Existing CI keeps Rust formatting/tests/Clippy on all three operating systems.
Packaging CI runs for relevant pull-request paths, manual dispatch, and releases.
Separate jobs build native packages and inspect/extract them, check formula
syntax, compare AUR metadata and build with makepkg, build/check Nix on Linux
and Apple Silicon, and build/smoke-test Snap and Flatpak. Packaging tool versions
are pinned in the workflow; update those independently of yoo's version.

## Local validation record

Validated on Linux x86_64:

- `cargo fmt --check`, `cargo test --locked` (57 tests),
  `cargo clippy --locked -- -D warnings`, `cargo build --release --locked`.
- `cargo deb --no-build`, `cargo generate-rpm`, package metadata/content
  inspection, extraction and `yoo --version` from both packages.
  This Arch host has no Debian package database; the local Debian validation
  supplied a temporary shlibs mapping based on the binary's GLIBC symbols.
  Release CI uses Ubuntu's real package database and asserts required libraries.
- `makepkg --verifysource`, `makepkg --nodeps`, and `.SRCINFO` comparison with
  `makepkg --printsrcinfo`, including generated publication metadata.
- `nix build`, `nix flake check` and `nix run ... -- --version` succeeded
  on x86_64 Linux using a clean source export and a user-space Nix store.
  All three systems evaluated; the macOS build is covered by CI.
- `ruby -c packaging/homebrew/yoo.rb`, shell syntax checks, YAML parsing and
  `actionlint` on GitHub workflows. Homebrew installation itself needs a
  supported Homebrew host and has not been exercised locally.
- Flatpak `flatpak-builder --show-manifest` and a successful offline Cargo build
  from the vendored source archive. Full SDK installation hit the workspace
  disk quota, so no local Flatpak bundle was produced.
- Snap YAML and Rust plugin options checked. Snapcraft's Python installation
  failed, and a system installation requires unavailable sudo access. No local
  snap was produced; the dedicated CI job builds and smoke-tests it.
- Release archive SHA-256 verification, metadata rendering, repeatable checksum
  regeneration and rejection of a mismatched release tag.

Local native/AUR packages and the publication bundle are under
`target/packaging-dist/`, with `SHA256SUMS`. These are validation outputs;
release CI rebuilds the native packages on Ubuntu 22.04.
