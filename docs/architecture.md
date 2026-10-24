# Code structure

`yoo` is a single Rust binary. `src/main.rs` parses arguments through `args.rs`,
then passes the command to `app::execute`. Parse errors exit with code 2;
execution errors exit with code 1. `app.rs` coordinates collection and display.

## Where to make changes

| Concern | Files |
| --- | --- |
| Commands, options, help | `src/args.rs`, `src/app.rs` |
| Shell completion generation | `src/completions.rs` |
| Project metadata and source counting | `src/project.rs`, `src/fetch.rs` |
| Git status | `src/git.rs` |
| Development tool checks | `src/doctor.rs` |
| Configuration and themes | `src/config.rs`, `src/ui.rs` |
| Saved project reports | `src/snapshot.rs` |
| Subprocess execution and cleanup | `src/process.rs`, `src/process_tree.rs` |
| Editor launch | `src/editor.rs` |
| Timers and tip packs | `src/timer.rs`, `src/tips.rs`, `src/content.rs` |

Keep command coordination in `app.rs` and collection logic in the relevant
module. Preserve structured JSON output when changing display behavior.
Subprocess collection uses timeouts, bounded captured output, and platform
specific process cleanup; preserve those guarantees when adding tool checks.
Interactive editor launches have different lifecycle requirements.

## Adding a command or option

1. Update argument parsing and help text in `args.rs`.
2. Connect execution in `app.rs` and implement the behavior in its module.
3. Update completion generation in `completions.rs` and regenerate the packaged
   scripts using `cargo build --release --locked` followed by
   `bash scripts/package-completions.sh`.
4. Add tests for observable behavior, including failure exit codes and JSON
   output where applicable. Use `tests/cli.rs` for compiled binary coverage.
5. Update `docs/commands.md` and the changelog when users will notice the change.

## Distribution boundaries

The website in `site/` and npm installer in `packages/npm/` are separate Node.js
projects. The website's installer route is generated from the root `install.sh`.
Package definitions live in `packaging/`, with generation and publication scripts
in `scripts/`. Signed repositories under `site/public/` contain release artifacts;
edit their source tooling instead of hand-editing signed metadata.

`Cargo.toml` uses an explicit package include list so website dependencies,
build output, and hosted packages stay out of the crate. Review
`cargo package --list --locked` when changing that boundary.
