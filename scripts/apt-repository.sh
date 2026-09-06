#!/usr/bin/env bash
# Build a signed repository from a .deb, retaining immutable packages and by-hash indexes.
set -euo pipefail
cd "$(dirname "$0")/.."
deb=$(realpath "${1:?usage: apt-repository.sh PACKAGE.deb [OUTPUT_DIRECTORY]}")
out=$(realpath -m "${2:-site/public/apt}")
: "${APT_SIGNING_FINGERPRINT:?set the dedicated signing key fingerprint}"
[[ "$APT_SIGNING_FINGERPRINT" =~ ^[A-F0-9]{40}$ ]] || { echo 'Invalid fingerprint' >&2; exit 1; }
[[ $(dpkg-deb -f "$deb" Package) == yoo && $(dpkg-deb -f "$deb" Architecture) == amd64 ]]
version=$(dpkg-deb -f "$deb" Version)
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+-[0-9]+$ ]] || { echo 'Unsupported package version' >&2; exit 1; }
if [[ -n "${APT_EXPECTED_VERSION:-}" ]]; then
  [[ "${version%-*}" == "$APT_EXPECTED_VERSION" ]] || { echo 'Package and release tag disagree' >&2; exit 1; }
fi
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
if [[ -d "$out" ]]; then cp -a "$out/." "$stage/"; fi
pool=pool/main/y/yoo
index=dists/stable/main/binary-amd64
mkdir -p "$stage/$pool" "$stage/$index/by-hash/SHA256" "$stage/$index/by-hash/SHA512"
for existing in "$stage/$pool/"*.deb; do
  [[ -f "$existing" ]] || continue
  previous=$(dpkg-deb -f "$existing" Version)
  dpkg --compare-versions "$version" ge "$previous" || { echo 'Refusing repository downgrade' >&2; exit 1; }
done
package="$pool/yoo_${version}_amd64.deb"
if [[ -f "$stage/$package" ]] && ! cmp -s "$deb" "$stage/$package"; then
  echo 'A published package version is immutable; increment the version or revision' >&2; exit 1
fi
install -m644 "$deb" "$stage/$package"
gpg --batch --export-options export-minimal --export "$APT_SIGNING_FINGERPRINT" > "$stage/public-key.gpg"
[[ -s "$stage/public-key.gpg" ]]
if [[ -f "$out/yoo-archive-keyring.gpg" ]]; then
  old=$(gpg --batch --show-keys --with-colons "$out/yoo-archive-keyring.gpg" | awk -F: '$1 == "fpr" {print $10; exit}')
  [[ "$old" == "$APT_SIGNING_FINGERPRINT" ]] || { echo 'Key rotation requires a separate reviewed migration' >&2; exit 1; }
fi
mv "$stage/public-key.gpg" "$stage/yoo-archive-keyring.gpg"
sed "s/@FINGERPRINT@/$APT_SIGNING_FINGERPRINT/g" packaging/apt/setup.sh.in > "$stage/setup.sh"
(
  cd "$stage"
  apt-ftparchive packages pool > "$index/Packages"
  gzip -n -9 -c "$index/Packages" > "$index/Packages.gz"
  for file in Packages Packages.gz; do
    for algorithm in 256 512; do
      hash=$("sha${algorithm}sum" "$index/$file" | cut -d ' ' -f1)
      install -m644 "$index/$file" "$index/by-hash/SHA$algorithm/$hash"
    done
  done
  # Never hash stale signatures or a previous Release into the new Release.
  rm -f dists/stable/Release dists/stable/Release.gpg dists/stable/InRelease
  apt-ftparchive \
    -o APT::FTPArchive::Release::Origin=yoo \
    -o APT::FTPArchive::Release::Label=yoo \
    -o APT::FTPArchive::Release::Suite=stable \
    -o APT::FTPArchive::Release::Codename=stable \
    -o APT::FTPArchive::Release::Architectures=amd64 \
    -o APT::FTPArchive::Release::Components=main \
    -o APT::FTPArchive::Release::Acquire-By-Hash=yes \
    -o "APT::FTPArchive::Release::Valid-Until=$(date -u -R -d '+30 days')" \
    release dists/stable > Release.new
  mv Release.new dists/stable/Release
  for mode in detached clear; do
    args=(--armor --detach-sign --output dists/stable/Release.gpg)
    if [[ "$mode" == clear ]]; then args=(--clearsign --output dists/stable/InRelease); fi
    printf '%s\n' "${APT_SIGNING_PASSPHRASE:-}" |
      gpg --batch --yes --pinentry-mode loopback --passphrase-fd 0 \
        --local-user "$APT_SIGNING_FINGERPRINT" --digest-algo SHA256 \
        "${args[@]}" dists/stable/Release
  done
  gpgv --keyring "$stage/yoo-archive-keyring.gpg" dists/stable/InRelease
  gpgv --keyring "$stage/yoo-archive-keyring.gpg" dists/stable/Release.gpg dists/stable/Release
)
# Vercel deploys this whole tree atomically; do not upload files individually.
mkdir -p "$out"
cp -a "$stage/." "$out/"
find "$out" -type d -exec chmod 755 {} +
find "$out" -type f -exec chmod 644 {} +
printf 'Signed yoo %s repository: %s\n' "$version" "$out"
