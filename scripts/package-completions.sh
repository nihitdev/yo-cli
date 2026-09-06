#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
binary=${1:-target/release/yoo}
mkdir -p packaging/completions
"$binary" completions bash > packaging/completions/yoo.bash
"$binary" completions zsh > packaging/completions/_yoo
"$binary" completions fish > packaging/completions/yoo.fish
