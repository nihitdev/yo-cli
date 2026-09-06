#!/usr/bin/env bash
# Refresh the vendored Cargo sources used by the openSUSE spec.
set -euo pipefail

cd "$(dirname "$0")/.."
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
cargo vendor "$tmp/vendor" >/dev/null
tar --zstd --sort=name --mtime='UTC 2026-01-01' --owner=0 --group=0 --numeric-owner \
  -cf packaging/opensuse/vendor.tar.zst -C "$tmp" vendor
printf 'Updated packaging/opensuse/vendor.tar.zst from Cargo.lock\n'
