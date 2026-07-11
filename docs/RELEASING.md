# Releasing yoo

Releases are built and published by GitHub Actions when a version tag is pushed.

1. Update the package version in `Cargo.toml` and refresh `Cargo.lock`.
2. Add the release notes to `CHANGELOG.md` and update versioned examples.
3. Run `cargo fmt --check`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo build --release --locked`.
4. Commit the release, create an annotated tag such as `v0.6.0`, and push the commit and tag.
5. Confirm that the Release workflow publishes Windows, Linux, and macOS archives plus `SHA256SUMS`.

Publishing the crate to crates.io remains a separate maintainer action using `cargo publish --locked`.
