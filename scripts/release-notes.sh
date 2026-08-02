#!/usr/bin/env bash

set -euo pipefail

tag="${1:?usage: release-notes.sh <tag> [changelog]}"
changelog="${2:-CHANGELOG.md}"
version="${tag#v}"

notes="$({
  awk -v version="$version" '
    $0 == "## [" version "]" || index($0, "## [" version "] - ") == 1 {
      found = 1
      next
    }
    found && /^## \[/ { exit }
    found { print }
    END { if (!found) exit 2 }
  ' "$changelog"
})" || {
  echo "No CHANGELOG.md section found for $version." >&2
  exit 1
}

if [[ -z "${notes//[[:space:]]/}" ]]; then
  echo "The CHANGELOG.md section for $version is empty." >&2
  exit 1
fi

printf '# yoo %s\n%s\n' "$version" "$notes"
