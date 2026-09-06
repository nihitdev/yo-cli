#!/usr/bin/env sh
# Regenerate the signed static Alpine repository from one release APK.
set -eu

apk_file=${1:?usage: alpine-repository.sh PACKAGE.apk [OUTPUT_DIRECTORY]}
output=${2:-site/public/alpine}
key_file=${ALPINE_SIGNING_KEY_FILE:?set ALPINE_SIGNING_KEY_FILE}
key_name=${ALPINE_SIGNING_KEY_NAME:-nihitdev@localhost-6a9d36e7.rsa.pub}

command -v apk >/dev/null || { echo 'apk is required' >&2; exit 1; }
command -v abuild-sign >/dev/null || { echo 'abuild-sign is required' >&2; exit 1; }
[ -f "$apk_file" ] || { echo "APK not found: $apk_file" >&2; exit 1; }

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT HUP INT TERM
mkdir -p "$stage/x86_64"
if [ -d "$output/x86_64" ]; then
  find "$output/x86_64" -maxdepth 1 -type f -name '*.apk' -exec cp {} "$stage/x86_64/" \;
fi
rm -f "$stage/x86_64/yoo-"*.apk
cp "$apk_file" "$stage/x86_64/"
apk --allow-untrusted index --rewrite-arch x86_64 -o "$stage/x86_64/APKINDEX.tar.gz" "$stage/x86_64/"*.apk
abuild-sign -k "$key_file" -p "$key_name" "$stage/x86_64/APKINDEX.tar.gz"
mkdir -p "$output/x86_64"
find "$output/x86_64" -maxdepth 1 -type f -name 'yoo-*.apk' -delete
cp "$stage/x86_64/"*.apk "$output/x86_64/"
cp "$stage/x86_64/APKINDEX.tar.gz" "$output/x86_64/APKINDEX.tar.gz"
printf 'Signed Alpine repository updated at %s\n' "$output"
