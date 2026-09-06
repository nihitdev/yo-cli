#!/usr/bin/env bash
set -euo pipefail
cd "${1:?usage: package-checksums.sh ARTIFACT_DIRECTORY}"
manifest=$(mktemp)
trap 'rm -f "$manifest"' EXIT
find . -maxdepth 1 -type f ! -name SHA256SUMS -printf '%f\0' |
  LC_ALL=C sort -z | xargs -0 -r sha256sum -- > "$manifest"
[[ -s "$manifest" ]] || { echo 'No artifacts to checksum' >&2; exit 1; }
install -m644 "$manifest" SHA256SUMS
