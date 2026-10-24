# Contributing to yoo

Thanks for wanting to improve `yoo`.

## Local setup

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli
cargo build --locked
cargo run -- --fast --name YourName
```

Install Rust through rustup, plus Git (used by the CLI and its tests). The
`rust-toolchain.toml` file selects stable Rust and installs rustfmt and Clippy.
The minimum supported Rust version (MSRV) is 1.85, declared in `Cargo.toml` and
tested separately in CI. To reproduce that check, install it with
`rustup toolchain install 1.85.0 --profile minimal`, then run
`cargo +1.85.0 test --locked`.

Run the CLI checks with Node.js 24 and Bash installed:

```bash
bash scripts/check.sh
```

The equivalent commands also work individually in PowerShell and other shells:

```bash
cargo fmt --check
cargo test-all
cargo lint
npm test --prefix packages/npm
bash scripts/check-shell.sh
node --test scripts/package-metadata.test.mjs
bash scripts/check-completions.sh
```

`cargo test-all` and `cargo lint` are repository aliases in `.cargo/config.toml`.
They use the committed lockfile, and linting covers all Rust targets with
warnings treated as errors. Run `cargo fmt` to apply formatting fixes.

## Website development

The Next.js website lives in `site/`. Use Node.js 24, matching CI:

```bash
cd site
npm ci
npm run dev
```

Before submitting website changes, run `npm run lint`,
`npx --no-install tsc --noEmit --incremental false`, and `npm run build` from
`site/`. Building regenerates the installer route from `install.sh`; commit
`site/app/yo-setup/installer.generated.ts` if it changes. The npm installer
wrapper and its tests live separately in `packages/npm/`.

The root and website `.node-version` files select Node.js 24 for compatible
version managers. The shell checks require Bash; on Windows, use Git Bash or
WSL for those two commands. Completion checks compare generated files without
rewriting tracked files and honor Cargo's configured target directory.

## Repository layout

See [Code structure](docs/architecture.md) for module responsibilities and the
steps for adding CLI commands.

- `src/` — CLI commands, project detection, configuration, and process handling.
- `tests/` — integration tests that run the compiled CLI.
- `docs/` and `examples/` — user documentation and sample configuration.
- `scripts/` and `packaging/` — release tooling and package definitions.
- `site/` — website and hosted package repositories.
- `packages/npm/` — npm binary installer and its tests.

## Before opening a pull request

- Keep each change focused.
- Add or update tests for behavior changes.
- Run `bash scripts/check.sh` (or the equivalent commands above).
- Update the README and changelog when users will notice the change.

Pushing a branch other than the default branch automatically opens a pull request.
Push additional commits to update that pull request; Dependabot branches are excluded.

## Community tip packs

Tip packs are YAML files. Keep them practical, short, original, and safe for a broad developer audience.

```yaml
name: web
description: Useful web-development reminders.
tips:
  - Test loading, error, and empty states.
  - Check the browser console before guessing.
```
