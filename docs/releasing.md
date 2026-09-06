# Releasing yoo

The canonical application version is the `version` field in `Cargo.toml`.
Keep it synchronized with the release tag, then run:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

The tag workflow validates the version, runs the Rust checks, builds the
Linux, macOS, and Windows release archives, generates SHA-256 checksums, and
creates the GitHub Release. It also builds the Debian, RPM, Snap, and Flatpak
artifacts, plus native Alpine, openSUSE, and Void packages, and attaches them
to the release. Generated AUR, Homebrew, WinGet, Scoop, and Chocolatey metadata
is included in `yoo-packaging.tar.gz`.

Repository publication jobs update the static repositories under
`site/public/` after the release. They skip safely when their credentials are
not configured and write the exact status to the Actions job summary.

## Optional Actions secrets

Existing repository secrets should retain their names:

| Secret | Purpose |
| --- | --- |
| `CARGO_REGISTRY_TOKEN` | Publish `yoo` to crates.io. |
| npm Trusted Publishing | No secret. Configure npm's trusted publisher for `@nihit_dev/yoo` with GitHub owner `nihitdev`, repository `yo-cli`, workflow `release.yml`, and no environment. |
| `CHOCOLATEY_API_KEY` | Submit the Windows package to Chocolatey moderation. |
| `APT_SIGNING_KEY`, `APT_SIGNING_PASSPHRASE`, `APT_PUBLISH_TOKEN` | Sign and publish the hosted APT repository. |
| `RPM_SIGNING_KEY`, `RPM_SIGNING_PASSPHRASE`, `RPM_PUBLISH_TOKEN` | Sign and publish Fedora and openSUSE repositories. |
| `HOMEBREW_TAP_TOKEN` | Update `nihitdev/homebrew-tap`. |
| `ALPINE_SIGNING_KEY`, `ALPINE_PUBLISH_TOKEN` | Sign and publish the hosted APK repository. |

Cross-repository submissions remain optional. If credentials are absent, the
workflow produces metadata artifacts and reports `SKIPPED - MISSING CREDENTIALS`
instead of failing unrelated release jobs. WinGet community submissions still
require a pull request to `microsoft/winget-pkgs`; the generated manifests are
available in the release metadata bundle for review.

The repository currently has no `nihitdev/scoop-bucket` remote to update, so
Scoop metadata is generated but not pushed. Void XBPS packages are built and
smoke-tested in Void Linux and attached to the GitHub Release, but there is no
official or hosted XBPS repository. Alpine APKs are built in an
Alpine container and attached to releases; hosted APKINDEX publication uses the
optional Alpine signing and publication secrets.

Never commit or print private keys. Repository update jobs use temporary key
directories and only commit public repository metadata.

## Local preflight

```bash
bash scripts/package-version.sh vX.Y.Z
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```
