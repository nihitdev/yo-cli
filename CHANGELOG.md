# Changelog

All notable changes to `yoo` are documented here.

## 0.5.0

### Added

- Added `yoo project` for a structured overview of the current project.
- Added project type and package-manager detection for Rust, Node.js, Python, Go, Java, and .NET projects.
- Added source-file and source-line counts while skipping generated folders such as `target`, `node_modules`, and `.git`.
- Added Git branch, working-tree, commit-count, and latest-tag information when available.
- Added checks for README, license, changelog, `.gitignore`, and GitHub Actions CI files.
- Added machine-readable output with `yoo project --json`.

## 0.4.0

### Added

- Added `yoo fetch` for developer environment and current-project information.
- Added `yoo status` as an alias for `yoo fetch`.
- Added JSON output with `yoo fetch --json`.
- Added project detection for Rust, Node.js, Python, Go, Java, and .NET markers.

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
