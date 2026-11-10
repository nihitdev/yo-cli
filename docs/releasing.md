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

After the GitHub Release is ready, npm publication uses trusted publishing and
crates.io publication uses `CARGO_REGISTRY_TOKEN`. Both skip versions that already
exist. Keep `packages/npm/package.json` and the README version example aligned
with `Cargo.toml` before tagging. Never publish the npm version before its GitHub
binary assets and `SHA256SUMS` are available.

After publication, the release workflow calls the Alpine, APT, and openSUSE
repository workflows in sequence to avoid competing default-branch pushes.
A failed repository publisher does not prevent the next publisher from running.
Each repository workflow serializes its runs, including manual and scheduled
publication. Different repositories can publish concurrently. Pushes rebase over
unrelated branch updates and retry up to five times. If the same signed repository changed remotely, publication
fails safely and must be rerun to regenerate its metadata.
The APT workflow also refreshes signatures weekly and supports manual dispatch.

Repository publication jobs update the static repositories under
`site/public/` after the release. They skip safely when their credentials are
not configured and write the exact status to the Actions job summary.

## Optional Actions secrets

Existing repository secrets should retain their names:

| Secret | Purpose |
| --- | --- |
| `CARGO_REGISTRY_TOKEN` | Publish `yoo` to crates.io. |
| `AUR_SSH_PRIVATE_KEY` | Publish `yoo-bin` to AUR over SSH. |
| npm Trusted Publishing | No secret. Configure npm's trusted publisher for `@nihit_dev/yoo` with GitHub owner `nihitdev`, repository `yo-cli`, workflow `release.yml`, and no environment. |
| `APT_SIGNING_KEY`, `APT_SIGNING_PASSPHRASE` | Sign and publish the hosted APT repository. |
| `RPM_SIGNING_KEY`, `RPM_SIGNING_PASSPHRASE` | Sign and publish the hosted openSUSE repository. |
| `ALPINE_SIGNING_KEY` | Sign the hosted APK repository. Publication uses the automatic `GITHUB_TOKEN` with `contents: write`. |

No repository variables are required. Homebrew and Chocolatey metadata is generated
without publishing credentials; neither ecosystem has a publisher job here.

Cross-repository submissions remain optional. If credentials are absent, the
workflow produces metadata artifacts and reports `SKIPPED - MISSING CREDENTIALS`
instead of failing unrelated release jobs. WinGet community submissions still
require a pull request to `microsoft/winget-pkgs`; the generated manifests are
available in the release metadata bundle for review.

The reviewed APT archive public key is tracked at
`packaging/apt/yoo-archive-keyring.gpg`. To rotate it, update that file and the
Actions signing secrets together, then explicitly dispatch the APT workflow
with `rotate_key: true`. The workflow checks that the imported private key
matches the reviewed public key, publishes a newly signed repository, and
commits its keyring and setup script atomically with the new repository
signatures. Existing APT clients must rerun the hosted setup command to trust
the replacement key.

The official Scoop bucket is
[`nihitdev/scoop-nihitdev`](https://github.com/nihitdev/scoop-nihitdev). It has its
own hourly manifest updater and a manual `Autoupdate manifests` workflow.
This release workflow includes Scoop metadata in the publication bundle;
the bucket updates independently. Void XBPS packages are built from the
checked-out source as an unprivileged user, install-tested in Void Linux, and
attached to the GitHub Release. There is no official or hosted XBPS repository.
Alpine APKs are built in an Alpine container and attached to releases; hosted APKINDEX publication uses the
optional Alpine signing key.

APT, Alpine, and openSUSE use the automatic `GITHUB_TOKEN` with `contents: write`
for same-repository updates. No `RPM_PUBLISH_TOKEN` is required. Branch rules must
permit these bot updates; the workflows do not bypass branch protection.

Alpine publishing only needs the signing key: GitHub issues a repository token
for each publishing job, so no `ALPINE_PUBLISH_TOKEN` personal access token is
required.

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
