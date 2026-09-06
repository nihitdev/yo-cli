#!/usr/bin/env bash
# Render publication files from the exact release archives, never placeholder hashes.
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(scripts/package-version.sh)
assets=$(realpath "${1:?usage: package-metadata.sh ASSET_DIRECTORY [OUTPUT_DIRECTORY]}")
out=${2:-packaging}
linux="yoo-v$version-linux-x86_64.tar.gz"
mac="yoo-v$version-macos-aarch64.tar.gz"
linux_sha=$(sha256sum "$assets/$linux" | cut -d ' ' -f1)
mac_sha=$(sha256sum "$assets/$mac" | cut -d ' ' -f1)
mkdir -p "$out/aur" "$out/homebrew"
for item in aur/PKGBUILD homebrew/yoo.rb; do
  sed -e "s/@VERSION@/$version/g" -e "s/@LINUX_SHA@/$linux_sha/g" \
    -e "s/@MAC_SHA@/$mac_sha/g" "packaging/$item.in" > "$out/$item"
done
# The fields below mirror PKGBUILD.in. CI compares this to makepkg --printsrcinfo.
cat > "$out/aur/.SRCINFO" <<SRCINFO
pkgbase = yoo-bin
	pkgdesc = Local CLI for project, Git, and development environment information
	pkgver = $version
	pkgrel = 1
	url = https://github.com/nihitdev/yo-cli
	arch = x86_64
	license = GPL-3.0-or-later
	depends = glibc
	depends = gcc-libs
	optdepends = git: Git repository information
	provides = yoo
	conflicts = yoo
	source = https://github.com/nihitdev/yo-cli/releases/download/v$version/$linux
	sha256sums = $linux_sha

pkgname = yoo-bin
SRCINFO
