#!/usr/bin/env bash
# Build yoo as an XBPS package inside a Void Linux container.
set -euo pipefail

workspace=${WORKSPACE:-/workspace}
void_packages=${VOID_PACKAGES_DIR:-/tmp/void-packages}

version=$(awk -F'"' '/^\[package\]/{p=1; next} p && /^version = /{print $2; exit}' "$workspace/Cargo.toml")
if [[ -n "${RELEASE_TAG:-}" ]]; then
  [[ "$RELEASE_TAG" == "v$version" ]]
fi

# Use the exact checkout for releases, pull requests, and manual builds.
source_archive="$workspace/target/yoo-void-source.tar.gz"
checksum=$(sha256sum "$source_archive" | cut -d ' ' -f1)
source_cache="$void_packages/hostdir/sources/yoo-$version"
mkdir -p "$source_cache"
cp "$source_archive" "$source_cache/v$version.tar.gz"
mkdir -p "$void_packages/srcpkgs/yoo"
sed -e "s/^version=.*/version=$version/" \
  -e "s/^checksum=.*/checksum=$checksum/" \
  "$workspace/packaging/void/template.in" > "$void_packages/srcpkgs/yoo/template"

cd "$void_packages"
./xbps-src binary-bootstrap
./xbps-src -Q pkg yoo
mkdir -p "$workspace/dist-void"
find hostdir/binpkgs -maxdepth 1 -type f -name 'yoo-*.xbps' -exec cp {} "$workspace/dist-void/" \;
package=$(find "$workspace/dist-void" -maxdepth 1 -type f -name 'yoo-*.xbps' -print -quit)
[[ -n "$package" ]]
tar -tf "$package" | grep -qx './usr/bin/yoo'
