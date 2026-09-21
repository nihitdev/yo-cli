# @nihit_dev/yoo

npm installer for the `yoo` local project and development-environment CLI.

```bash
npm install -g @nihit_dev/yoo
yoo
```

This package installs the prebuilt binary from the matching GitHub release:

- Windows x64
- Linux x64
- macOS arm64

The Rust crate is available as `yoo` on crates.io:

```bash
cargo install yoo
```

Downloads are checked against the matching release's `SHA256SUMS` before the
binary is atomically installed. Failed downloads leave any existing executable
untouched. Downloads require HTTPS, allow up to five redirects, and have a
60-second deadline per file. No additional npm dependencies are required.

For local builds, set `YOO_BINARY_PATH` to a nonempty native executable. This
explicit override copies your file without downloading or checking release hashes.

Maintainers can run the local HTTP-fixture regression suite with `npm test`.
