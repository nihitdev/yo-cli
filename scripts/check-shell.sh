#!/usr/bin/env bash
# Parse shell scripts without running installers or publication commands.
set -euo pipefail
cd "$(dirname "$0")/.."

while IFS= read -r -d '' file; do
  bash -n "$file"
done < <(git ls-files --cached --others --exclude-standard -z -- '*.sh')
