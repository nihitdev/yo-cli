#!/usr/bin/env bash
# Build a signed RPM-MD repository for openSUSE from one release RPM.
set -euo pipefail

cd "$(dirname "$0")/.."
rpm_file=$(realpath "${1:?usage: opensuse-repository.sh PACKAGE.rpm [OUTPUT_DIRECTORY]}")
output=$(realpath -m "${2:-site/public/opensuse}")
: "${RPM_SIGNING_KEY_ID:?set the RPM signing key fingerprint or key id}"
: "${RPM_SIGNING_PASSPHRASE:?set the RPM signing passphrase}"

command -v rpm >/dev/null || { echo 'rpm is required' >&2; exit 1; }
command -v createrepo_c >/dev/null || { echo 'createrepo_c is required' >&2; exit 1; }
command -v gpg >/dev/null || { echo 'gpg is required' >&2; exit 1; }

[[ "$(rpm -qp --qf '%{NAME}' "$rpm_file")" == yoo ]]
[[ "$(rpm -qp --qf '%{ARCH}' "$rpm_file")" == x86_64 ]]
version=$(rpm -qp --qf '%{VERSION}-%{RELEASE}' "$rpm_file")
package_name=$(basename "$rpm_file")
[[ "$package_name" == yoo-*.rpm ]] || { echo 'unexpected RPM filename' >&2; exit 1; }

staging=$(mktemp -d)
passfile=$(mktemp)
chmod 600 "$passfile"
printf '%s' "$RPM_SIGNING_PASSPHRASE" > "$passfile"
trap 'rm -f "$passfile"; rm -rf "$staging"' EXIT HUP INT TERM
if [[ -d "$output" ]]; then cp -a "$output/." "$staging/"; fi
mkdir -p "$staging/packages"
destination="$staging/packages/$package_name"

cp "$rpm_file" "$destination"
rpm --define "_gpg_name $RPM_SIGNING_KEY_ID" \
  --define "_gpg_sign_cmd_extra_args --batch --pinentry-mode loopback --passphrase-file $passfile" \
  --resign "$destination"
rpm --checksig --verbose "$destination"

public_fingerprint=$(gpg --batch --show-keys --with-colons site/public/rpm/RPM-GPG-KEY-yoo |
  awk -F: '$1 == "fpr" {print $10; exit}')
[[ "$public_fingerprint" == "$RPM_SIGNING_KEY_ID" ]] || {
  echo 'RPM_SIGNING_KEY_ID does not match the committed public key' >&2
  exit 1
}
cp site/public/rpm/RPM-GPG-KEY-yoo "$staging/RPM-GPG-KEY-yoo"
cp packaging/opensuse/yoo.repo "$staging/yoo.repo"
createrepo_c --update --checksum sha256 --unique-md-filenames "$staging"

gpg --batch --yes --pinentry-mode loopback --passphrase-file "$passfile" \
  --local-user "$RPM_SIGNING_KEY_ID" --digest-algo SHA256 --armor --detach-sign \
  --output "$staging/repodata/repomd.xml.asc" "$staging/repodata/repomd.xml"
gpgv --keyring <(gpg --batch --export "$RPM_SIGNING_KEY_ID") \
  "$staging/repodata/repomd.xml.asc" "$staging/repodata/repomd.xml"

mkdir -p "$output"
cp -a "$staging/." "$output/"
find "$output" -type d -exec chmod 755 {} +
find "$output" -type f -exec chmod 644 {} +
printf 'Signed openSUSE repository %s at %s\n' "$version" "$output"
