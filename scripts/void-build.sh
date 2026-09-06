#!/usr/bin/env bash
# Build yoo as an XBPS package inside a Void Linux container.
set -euo pipefail

version=$(awk -F'"' '/^\[package\]/{p=1; next} p && /^version = /{print $2; exit}' /workspace/Cargo.toml)
if [[ -n "${RELEASE_TAG:-}" ]]; then
  [[ "$RELEASE_TAG" == "v$version" ]]
fi

wget -q "https://github.com/nihitdev/yo-cli/archive/refs/tags/v$version.tar.gz" -O /tmp/yoo-source.tar.gz
checksum=$(sha512sum /tmp/yoo-source.tar.gz | cut -d ' ' -f1)
mkdir -p /tmp/void-packages/srcpkgs/yoo
sed -e "s/^version=.*/version=$version/" \
  -e "s/^checksum=.*/checksum=$checksum/" \
  /workspace/packaging/void/template.in > /tmp/void-packages/srcpkgs/yoo/template

cd /tmp/void-packages
./xbps-src binary-bootstrap
./xbps-src pkg yoo
mkdir -p /workspace/dist-void
find hostdir/binpkgs -maxdepth 1 -type f -name 'yoo-*.xbps' -exec cp {} /workspace/dist-void/ \;
package=$(find /workspace/dist-void -maxdepth 1 -type f -name 'yoo-*.xbps' -print -quit)
[[ -n "$package" ]]
tar -tf "$package" | grep -qx './usr/bin/yoo'
xbps-install -y -R /tmp/void-packages/hostdir/binpkgs yoo
[[ "$(yoo --version)" == "yoo $version" ]]
