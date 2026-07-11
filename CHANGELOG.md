# Changelog

All notable changes to `yoo` are documented here.

## Unreleased

### Added

- Added `yoo version` as a command-form equivalent of `yoo --version`.

### Changed

- Redirected output no longer contains ANSI styling or waits for typewriter animation.
- CI now uses the committed dependency lockfile for tests and Clippy.

### Fixed

- Source counting skips symlinks and recognises source extensions case-insensitively.
- Local tip packs recognise `.yaml` and `.yml` extensions regardless of letter case.

## 0.6.1

### Removed

- Removed the TOML and XML parser dependencies added in 0.6.0 to keep the executable lean.
- Removed the expanded multi-language manifest metadata and project-aware doctor behavior.

### Changed

- Cargo package fields use a small dependency-free reader that stays inside the `[package]` section and supports quoted values.

## 0.6.0

### Added

- Added structured Cargo, Python, Node.js, Go, Maven, and .NET manifest metadata readers.
- Added project-aware `yoo doctor` checks for Rust, Node.js, Python, Go, Java, and .NET toolchains.
- Added end-to-end CLI tests for version reporting, argument errors, and JSON project detection.
- Added tag-driven GitHub Releases with Windows, Linux, and macOS archives plus SHA-256 checksums.

### Changed

- External Git and tool probes now stop after a bounded timeout instead of being able to hang indefinitely.
- Cargo metadata now supports quoted TOML variants and workspace-inherited package fields.

### Fixed

- Fixed the malformed `*.pdb` ignore rule introduced while untracking generated distribution files.

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
