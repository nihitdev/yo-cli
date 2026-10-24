#!/usr/bin/env bash
# Run from any directory. Website checks remain in the separate CI job.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo fmt --check
cargo test-all
cargo lint
npm test --prefix packages/npm
bash scripts/check-shell.sh
bash scripts/check-completions.sh
