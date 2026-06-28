# Contributing to yoo

Thanks for wanting to improve `yoo`.

## Local setup

```bash
git clone https://github.com/nihitdev/yo-cli.git
cd yo-cli
cargo fmt
cargo test
cargo clippy -- -D warnings
cargo run -- --fast --name YourName
```

## Before opening a pull request

- Keep each change focused.
- Add or update tests for behavior changes.
- Run `cargo fmt`, `cargo test`, and `cargo clippy -- -D warnings`.
- Update the README and changelog when users will notice the change.

## Community tip packs

Tip packs are YAML files. Keep them practical, short, original, and safe for a broad developer audience.

```yaml
name: web
description: Useful web-development reminders.
tips:
  - Test loading, error, and empty states.
  - Check the browser console before guessing.
```
