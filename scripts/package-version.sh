#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(sed -n '/^\[package\]/,/^\[/s/^version = "\([^"]*\)"/\1/p' Cargo.toml)
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Packaging requires a stable Cargo version" >&2; exit 1; }
if [[ -n "${GITHUB_REF_NAME:-}" && "${GITHUB_REF_TYPE:-}" == tag ]]; then
  [[ "$GITHUB_REF_NAME" == "v$version" ]] || { echo "Tag and Cargo version disagree" >&2; exit 1; }
fi
printf '%s\n' "$version"
