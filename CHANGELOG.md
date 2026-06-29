# Changelog

All notable changes to `yoo` are documented here.

## 0.4.0

- Added `yoo fetch`, a developer-aware environment and project status command.
- Added `yoo status` as an alias for `yoo fetch`.
- Added detection for OS, architecture, shell, terminal, editor, Rust, Cargo, and Git.
- Added project detection for Rust, Node.js, Python, Go, Java, and .NET repositories.
- Added current Git branch and working-tree status to fetch output.
- Added `yoo fetch --json` for machine-readable output.
- Added display controls for fetch: `--theme`, `--plain`, and `--no-art`.

## 0.3.0

- Added YAML configuration at the standard OS config location.
- Added `yoo doctor` for Rust, Cargo, Git, config, and current-project checks.
- Added a local coding-session timer with `yoo session [MINUTES]`.
- Added nine themes: neon, ocean, mono, dracula, tokyo-night, gruvbox, nord, rose-pine, and catppuccin.
- Added built-in tip packs: general, rust, git, and linux.
- Added community YAML tip packs stored in the user config directory.
- Added `yoo tip [PACK]` and `yoo tips`.
- Switched the project license to GPL-3.0-or-later.

## 0.2.0

- Added configurable name, themes, typewriter mode, Git information, tests, and CI.

## 0.1.0

- Initial crates.io release.
