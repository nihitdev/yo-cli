#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(sed -n '/^\[package\]/,/^\[/s/^version = "\([^"]*\)"/\1/p' Cargo.toml)
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Packaging requires a stable Cargo version" >&2; exit 1; }
tag=''
if [[ $# -gt 0 ]]; then
  tag=$1
elif [[ "${GITHUB_REF_TYPE:-}" == tag ]]; then
  tag=${GITHUB_REF_NAME:-}
fi
if [[ -n "$tag" ]]; then
  [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Release tag must look like vX.Y.Z" >&2; exit 1; }
  [[ "$tag" == "v$version" ]] || { echo "Tag and Cargo version disagree: $tag vs v$version" >&2; exit 1; }
fi
printf '%s\n' "$version"
