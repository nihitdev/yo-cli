#!/usr/bin/env sh
# Build the current checkout as an Alpine package.
set -eu

version=$(awk -F\" '/^\[package\]/{p=1; next} p && /^version = /{print $2; exit}' /workspace/Cargo.toml)

if [ -n "${RELEASE_TAG:-}" ]; then
  [ "$RELEASE_TAG" = "v$version" ]
fi

# Package the checkout being tested instead of downloading an already
# published GitHub tag. This keeps PR validation tied to the actual source.
source_dir="/tmp/yo-cli-$version"
rm -rf "$source_dir"
mkdir -p "$source_dir"

tar \
  --exclude='.git' \
  --exclude='target' \
  --exclude='dist-alpine' \
  -C /workspace \
  -cf - . |
tar -C "$source_dir" -xf -

tar -czf "/tmp/yoo-$version.tar.gz" -C /tmp "yo-cli-$version"
checksum=$(sha512sum "/tmp/yoo-$version.tar.gz" | cut -d ' ' -f1)

sed -E \
  -e "s/^pkgver=.*/pkgver=$version/" \
  -e "s|^source=.*|source=\"yoo-$version.tar.gz\"|" \
  -e "s/^[0-9a-f]+  yoo-.*\.tar\.gz$/$checksum  yoo-$version.tar.gz/" \
  /workspace/packaging/alpine/APKBUILD \
  > /home/builder/packages/yoo/APKBUILD

cp "/tmp/yoo-$version.tar.gz" /home/builder/packages/yoo/
chown -R builder:builder /home/builder

su builder -c 'abuild-keygen -a -n'
cp /home/builder/.abuild/*.rsa.pub /etc/apk/keys/

su builder -c 'cd /home/builder/packages/yoo && abuild -r'

find /home/builder/packages \
  -name 'yoo-*.apk' \
  -exec cp {} /workspace/dist-alpine/ \;
