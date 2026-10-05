# Command reference

[← Back to yoo](../README.md)

Commands inspect the current directory. Use `cd path/to/project` first; positional project paths are not supported.

| Command | Purpose |
| --- | --- |
| `yoo` | Session summary with project and Git context |
| `yoo doctor` | Check local tools, project detection, and configuration |
| `yoo project` | Project metadata, source counts, and Git details |
| `yoo snapshot` | Save a local snapshot of the current project |
| `yoo snapshot list` | List saved snapshots in reverse chronological order |
| `yoo snapshot compare` | Compare the two newest local snapshots |
| `yoo snapshot compare OLD_ID NEW_ID` | Compare selected saved snapshots |
| `yoo snapshot compare [ID] --current` | Compare a saved snapshot (latest by default) with the current project |
| `yoo fetch` | Project plus development environment report |
| `yoo status` | Alias for `yoo fetch` (environment and project report) |
| `yoo session [MINUTES]` | Start a local coding-session timer |
| `yoo tip` / `yoo tips` | Show a configured reminder or tip pack |
| `yoo edit` | Open the current project in the configured editor |
| `yoo init` / `yoo config` | Create configuration and a sample tip pack / print the configuration file path |
| `yoo completions SHELL` | Generate Bash, Zsh, Fish, or PowerShell completions |

Useful output flags include `--fast`, `--plain`, `--no-art`, `--theme NAME`, and `--json` for `project` and `fetch`.

```bash
yoo doctor
yoo project --json
yoo fetch --plain
yoo snapshot
# make changes, then capture another point in time
yoo snapshot
yoo snapshot list
yoo snapshot compare
yoo session 25
```

Snapshots are saved as JSON files in `.yoo/snapshots/` inside the current
project. On the first save, yoo creates `.yoo/snapshots/.gitignore` containing
`*`, so snapshots and that ignore file stay out of Git without changing your
project’s ignore rules. An existing snapshot ignore file is preserved. `snapshot compare` compares the two newest saved snapshots;
save another snapshot after making changes to track progress over time.
`snapshot list` prints each snapshot's numeric ID. Use those IDs to select a
comparison, or run `yoo snapshot compare --current` to compare the latest saved
snapshot with the current project without writing a new snapshot. Comparisons
also show changes to the latest commit.

Check the installed CLI version with `yoo --version`.

`project --json` and `fetch --json` include `yoo_version`. Their `project.version`
field is the inspected project's version, when available. `--json` cannot be
combined with `--plain`, `--no-art`, or `--theme`.

Git inspection distinguishes clean and dirty repositories, repositories before
their first commit, and non-Git directories. Git failures and timeouts are
reported on stderr with exit status 1 (including in JSON mode), rather than
being reported as a clean tree. Exit status 2 indicates invalid CLI arguments.
Git reports include ahead/behind counts when an upstream is configured, active
rebase/merge/cherry-pick/revert/bisect operations, conflicts, and tracked deletions.
Counts use local refs; yoo does not fetch. Ten or more tracked deletions trigger
an advisory warning to review before staging. `doctor` reports conflicts and
active operations as warnings; yoo does not attempt Git recovery. JSON reports
include these details under `git.diagnostics`.

`yoo doctor` prints all checks and exits 1 if any check fails; warnings alone
exit 0. Git is checked for every project.
Checks now follow the detected project: Rust checks Rust, Cargo, Rustfmt and
Clippy; Node.js checks Node and the package manager selected by its lockfile;
Python, Go, Java, .NET, Zig, Ruby, PHP, Swift, Dart and Elixir check their
respective runtime or SDK; CMake projects check CMake. Ruby, PHP and Elixir
also check Bundler, Composer and Mix respectively. Generic
directories check Git without requiring a language toolchain.
These are development health checks, not dependencies needed to launch `yoo`.

Source totals include all supported source extensions across languages, respect
nested `.gitignore` rules, `.git/info/exclude`, and global Git excludes, and skip
symlinks and common generated/vendor directories (`target`, `node_modules`,
`dist`, `build`, `.next`, `.venv`, `venv`, `vendor`, `__pycache__`, `.git`). Counts
also honor `.ignore` files. Files that cannot be read are skipped.
