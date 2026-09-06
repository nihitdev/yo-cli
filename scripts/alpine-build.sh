#!/usr/bin/env sh
# Build the release APK inside an Alpine container.
set -eu

version=$(awk -F\" '/^\[package\]/{p=1; next} p && /^version = /{print $2; exit}' /workspace/Cargo.toml)
if [ -n "${RELEASE_TAG:-}" ]; then
  [ "$RELEASE_TAG" = "v$version" ]
fi
wget -q "https://github.com/nihitdev/yo-cli/archive/refs/tags/v$version.tar.gz" -O /tmp/yoo-source.tar.gz
checksum=$(sha512sum /tmp/yoo-source.tar.gz | cut -d ' ' -f1)
sed -E -e "s/^pkgver=.*/pkgver=$version/" \
  -e "s/^[0-9a-f]+  yoo-.*\.tar\.gz$/$checksum  yoo-$version.tar.gz/" \
  /workspace/packaging/alpine/APKBUILD > /home/builder/packages/yoo/APKBUILD
chown -R builder:builder /home/builder
su builder -c 'abuild-keygen -a -n'
cp /home/builder/.abuild/*.rsa.pub /etc/apk/keys/
su builder -c 'cd /home/builder/packages/yoo && abuild -r'
find /home/builder/packages -name 'yoo-*.apk' -exec cp {} /workspace/dist-alpine/ \;
