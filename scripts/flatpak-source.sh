#!/usr/bin/env bash
# Vendor locked dependencies before Flatpak's network-isolated build.
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(scripts/package-version.sh)
mkdir -p target/flatpak
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/yoo-$version"
cp Cargo.toml Cargo.lock LICENSE README.md "$stage/yoo-$version/"
cp -R src "$stage/yoo-$version/"
cargo vendor --locked "$stage/yoo-$version/vendor" > "$stage/cargo-config.toml"
# cargo vendor emits an absolute directory; the archive must be relocatable.
sed 's|^directory = .*|directory = "vendor"|' "$stage/cargo-config.toml" > "$stage/yoo-$version/cargo-config.toml"
tar --sort=name --mtime="@${SOURCE_DATE_EPOCH:-0}" --owner=0 --group=0 --numeric-owner \
  -czf target/flatpak/yoo-source.tar.gz -C "$stage" "yoo-$version"
