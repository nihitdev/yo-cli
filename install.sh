#!/bin/sh

set -eu

repository="nihitdev/yo-cli"
install_dir="${YOO_INSTALL_DIR:-$HOME/.local/bin}"
version="${YOO_VERSION:-latest}"

fail() {
  printf 'yoo installer: %s\n' "$1" >&2
  exit 1
}

command_exists() {
  command -v "$1" >/dev/null 2>&1
}

download() {
  url=$1
  destination=$2

  if command_exists curl; then
    curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error \
      "$url" --output "$destination"
  elif command_exists wget; then
    wget --https-only --quiet "$url" --output-document "$destination"
  else
    fail "curl or wget is required"
  fi
}

os=$(uname -s)
architecture=$(uname -m)

case "$os:$architecture" in
  Linux:x86_64|Linux:amd64)
    asset="yoo-linux-x86_64"
    ;;
  Darwin:arm64|Darwin:aarch64)
    asset="yoo-macos-aarch64"
    ;;
  *)
    fail "unsupported platform $os/$architecture; use Cargo or npm instead"
    ;;
esac

if [ "$version" = "latest" ]; then
  base_url="https://github.com/$repository/releases/latest/download"
else
  version=${version#v}
  case "$version" in
    *[!0-9.]*|'') fail "YOO_VERSION must look like 0.7.0" ;;
  esac
  base_url="https://github.com/$repository/releases/download/v$version"
fi

temporary_directory=$(mktemp -d 2>/dev/null || mktemp -d -t yoo-install)
trap 'rm -rf "$temporary_directory"' EXIT HUP INT TERM

binary="$temporary_directory/$asset"
checksums="$temporary_directory/SHA256SUMS"

printf 'Downloading %s...\n' "$asset"
download "$base_url/$asset" "$binary"
download "$base_url/SHA256SUMS" "$checksums"

expected_checksum=$(awk -v asset="$asset" '$2 == asset || $2 == "*" asset { print $1; exit }' "$checksums")
[ -n "$expected_checksum" ] || fail "release checksum for $asset was not found"

if command_exists sha256sum; then
  actual_checksum=$(sha256sum "$binary" | awk '{ print $1 }')
elif command_exists shasum; then
  actual_checksum=$(shasum -a 256 "$binary" | awk '{ print $1 }')
else
  fail "sha256sum or shasum is required to verify the download"
fi

[ "$actual_checksum" = "$expected_checksum" ] || fail "download checksum did not match"

mkdir -p "$install_dir"
chmod 755 "$binary"
mv "$binary" "$install_dir/yoo"

printf 'Installed yoo to %s/yoo\n' "$install_dir"
case ":$PATH:" in
  *":$install_dir:"*) ;;
  *) printf 'Add %s to PATH to run yoo from any directory.\n' "$install_dir" ;;
esac
