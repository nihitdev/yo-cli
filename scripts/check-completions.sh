#!/usr/bin/env bash
# Compare generated completions without modifying the working tree.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --locked
binary=$(cargo metadata --locked --no-deps --format-version 1 | node -e '
let input = "";
process.stdin.on("data", chunk => input += chunk);
process.stdin.on("end", () => {
  const path = require("node:path");
  const name = process.platform === "win32" ? "yoo.exe" : "yoo";
  console.log(path.join(JSON.parse(input).target_directory, "debug", name));
});
')
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT

for shell in bash zsh fish; do
  case "$shell" in
    bash) file=yoo.bash ;;
    zsh) file=_yoo ;;
    fish) file=yoo.fish ;;
  esac
  "$binary" completions "$shell" > "$temporary/$file"
  if ! diff -u "packaging/completions/$file" "$temporary/$file"; then
    echo 'Packaged completions are stale. Regenerate with:' >&2
    echo '  cargo build --release --locked && bash scripts/package-completions.sh' >&2
    exit 1
  fi
done
