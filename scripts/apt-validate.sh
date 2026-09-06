#!/usr/bin/env bash
# Exercise APT's signature and package verification without altering system sources.
set -euo pipefail
repo=$(realpath "${1:-site/public/apt}")
[[ ! "$repo" =~ [[:space:]] ]] || { echo 'Validation path must not contain whitespace' >&2; exit 1; }
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
chmod 755 "$work"
mkdir -p "$work/lists/partial" "$work/cache/archives/partial" "$work/download"
touch "$work/status"
gpgv --keyring "$repo/yoo-archive-keyring.gpg" --output "$work/Release" "$repo/dists/stable/InRelease"
cmp "$work/Release" "$repo/dists/stable/Release"
gpgv --keyring "$repo/yoo-archive-keyring.gpg" "$repo/dists/stable/Release.gpg" "$repo/dists/stable/Release"
(cd "$repo" && apt-ftparchive packages pool) > "$work/Packages"
cmp "$work/Packages" "$repo/dists/stable/main/binary-amd64/Packages"
gzip -dc "$repo/dists/stable/main/binary-amd64/Packages.gz" | cmp - "$work/Packages"
cat > "$work/yoo.sources" <<SOURCES
Types: deb
URIs: file:$repo
Suites: stable
Components: main
Architectures: amd64
Signed-By: $repo/yoo-archive-keyring.gpg
SOURCES
options=(
  -o "Dir::Etc::sourcelist=$work/yoo.sources" -o Dir::Etc::sourceparts=-
  -o "Dir::State::lists=$work/lists" -o "Dir::State::status=$work/status"
  -o "Dir::Cache=$work/cache" -o "APT::Sandbox::User=$(id -un)"
  -o APT::Architecture=amd64 -o APT::Architectures::=amd64
  -o APT::Update::Error-Mode=any
)
# An optional local tools path is useful on non-Debian development hosts.
if [[ -n "${APT_METHODS:-}" ]]; then options+=(-o "Dir::Bin::Methods=$APT_METHODS"); fi
apt-get "${options[@]}" update
apt-cache "${options[@]}" policy yoo
(cd "$work/download" && apt-get "${options[@]}" download yoo)
package=("$work/download/"*.deb)
[[ ${#package[@]} == 1 && -f "${package[0]}" ]]
version=$(dpkg-deb -f "${package[0]}" Version)
cmp "${package[0]}" "$repo/pool/main/y/yoo/yoo_${version}_amd64.deb"
echo "APT authenticated and downloaded yoo $version"
