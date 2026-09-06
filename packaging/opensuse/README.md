# openSUSE Tumbleweed packaging

`yoo.spec` builds the x86_64 package from the tagged GitHub source archive with
the checked-in `Cargo.lock`. `vendor.tar.zst` makes the build independent of
network access after sources are prepared. The spec uses openSUSE's
`cargo-packaging` macros and runs the locked test suite.

When `Cargo.lock` changes, refresh the source bundle before building:

```bash
bash scripts/opensuse-vendor.sh
```

The static repository at `site/public/opensuse/` is RPM-MD metadata generated
by `createrepo_c`. Both the RPM and `repomd.xml` are signed with the existing
dedicated yoo RPM key. The public key is safe to commit; the private key never
belongs in Git.

Build locally on Tumbleweed:

```bash
sudo zypper install cargo cargo-packaging createrepo_c rpm-build gpg2
rpmbuild -ba packaging/opensuse/yoo.spec
```

Regenerate the hosted repository from a built RPM:

```bash
export RPM_SIGNING_KEY_ID=YOUR_YOO_KEY_FINGERPRINT
export RPM_SIGNING_PASSPHRASE='read this from a password manager'
bash scripts/opensuse-repository.sh ~/rpmbuild/RPMS/x86_64/yoo-1.0.0-1.x86_64.rpm
```

The script preserves older packages, refreshes `repodata/`, creates unique
metadata filenames, signs the repository metadata, and refuses non-`yoo`
x86_64 RPMs. For a release, CI obtains the RPM artifact, signs it, then commits
the public repository tree to the website branch for Vercel's normal deployment.

Install after that deployment:

```bash
sudo zypper ar -f https://yo-cli.vercel.app/opensuse yoo
sudo zypper refresh
sudo zypper install yoo
```

The repository keeps GPG checks enabled. Do not use `--no-gpg-checks`.

## Signing key setup

Create a dedicated key in an isolated temporary GnuPG home, then export the
armored private key into the GitHub Actions secret. Keep the passphrase in a
separate secret and destroy the temporary home after export:

```bash
GNUPGHOME=$(mktemp -d)
chmod 700 "$GNUPGHOME"
export GNUPGHOME
passphrase=$(openssl rand -base64 36)
printf '%s\n' "$passphrase" > key-passphrase.txt
gpg --batch --pinentry-mode loopback --passphrase-file key-passphrase.txt \
  --quick-generate-key 'yoo RPM Repository <nihitdev@users.noreply.github.com>' rsa4096 sign 2y
gpg --list-secret-keys --with-colons
gpg --armor --export-secret-keys YOUR_FINGERPRINT > yoo-rpm-private.asc
gpg --armor --export YOUR_FINGERPRINT > site/public/opensuse/RPM-GPG-KEY-yoo
```

Set these repository secrets:

- `RPM_SIGNING_KEY`: contents of `yoo-rpm-private.asc`.
- `RPM_SIGNING_PASSPHRASE`: the key's passphrase.
- `RPM_PUBLISH_TOKEN`: a fine-grained GitHub token with Contents: Read and write
  on `nihitdev/yo-cli`, used only by the release job to commit
  `site/public/opensuse`.

The existing `site/public/rpm/RPM-GPG-KEY-yoo` is the current public key. Copy it
to the openSUSE path only when its fingerprint is the intended dedicated key.
Rotate keys through a reviewed migration: publish the new public key and keep
the old key available until clients have refreshed their repository definition.
