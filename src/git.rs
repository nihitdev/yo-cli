use std::{path::Path, process::Command};

use crate::process::{self, CommandError, ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitInfo {
    pub branch: String,
    pub changed_files: usize,
    pub diagnostics: GitDiagnostics,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct GitDiagnostics {
    pub upstream: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub operation: Option<String>,
    pub conflicts: usize,
    pub deleted_files: usize,
    pub large_deletion: bool,
}

impl GitDiagnostics {
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if let Some(upstream) = &self.upstream {
            parts.push(format!(
                "{upstream}: {} ahead, {} behind (local refs)",
                self.ahead, self.behind
            ));
        }
        if let Some(operation) = &self.operation {
            parts.push(format!("{operation} in progress"));
        }
        if self.conflicts > 0 {
            parts.push(format!("{} conflicted file(s)", self.conflicts));
        }
        if self.deleted_files > 0 {
            parts.push(format!("{} tracked deletion(s)", self.deleted_files));
        }
        if self.large_deletion {
            parts.push("WARNING: unusually large tracked deletion; review before staging".into());
        }
        parts.join("; ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
}

/// `Ok(None)` means positively identified as a non-repository. Command errors,
/// including missing Git and timeouts, must never be reported as a clean tree.
pub fn inspect(directory: &Path) -> Result<Option<GitInfo>, CommandError> {
    let mut info = inspect_with(|arguments| run_git(directory, arguments))?;
    if let Some(info) = &mut info {
        for (marker, operation) in [
            ("rebase-merge", "rebase"),
            ("rebase-apply", "rebase/apply"),
            ("MERGE_HEAD", "merge"),
            ("CHERRY_PICK_HEAD", "cherry-pick"),
            ("REVERT_HEAD", "revert"),
            ("BISECT_LOG", "bisect"),
        ] {
            let path = run_git(directory, &["rev-parse", "--git-path", marker])?;
            if directory.join(path.trim()).exists() {
                info.diagnostics.operation = Some(operation.into());
                break;
            }
        }
        // --porcelain=v2 provides upstream counts without fetching or failing
        // for unborn branches, detached HEAD, or branches without an upstream.
        let status = run_git(
            directory,
            &[
                "status",
                "--porcelain=v2",
                "--branch",
                "-z",
                "--untracked-files=no",
            ],
        )?;
        for line in status.split('\0') {
            if let Some(value) = line.strip_prefix("# branch.upstream ") {
                info.diagnostics.upstream = Some(value.into());
            }
            if let Some(value) = line.strip_prefix("# branch.ab ") {
                let mut counts = value.split_whitespace();
                info.diagnostics.ahead = counts
                    .next()
                    .and_then(|v| v.strip_prefix('+'))
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
                info.diagnostics.behind = counts
                    .next()
                    .and_then(|v| v.strip_prefix('-'))
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0);
            }
        }
    }
    Ok(info)
}

fn inspect_with(
    mut run: impl FnMut(&[&str]) -> Result<String, CommandError>,
) -> Result<Option<GitInfo>, CommandError> {
    match run(&["rev-parse", "--git-dir"]) {
        Ok(_) => {}
        Err(error)
            if error.kind == ErrorKind::Failed
                && error
                    .stderr
                    .starts_with("fatal: not a git repository (or any") =>
        {
            return Ok(None);
        }
        Err(error) => return Err(error),
    }
    // symbolic-ref works before the first commit. Detached HEAD is the only
    // expected failure (exit 1); other failures must remain visible.
    let branch = match run(&["symbolic-ref", "--quiet", "--short", "HEAD"]) {
        Ok(branch) => branch,
        Err(error) if error.kind == ErrorKind::Failed && error.exit_code == Some(1) => {
            run(&["rev-parse", "--short", "HEAD"])?
        }
        Err(error) => return Err(error),
    };
    let status = run(&["status", "--porcelain=v1", "-z", "--untracked-files=all"])?;
    // With -z a rename/copy has two path records. Paths can contain newlines.
    let mut records = status.split('\0').filter(|record| !record.is_empty());
    let mut changed_files = 0;
    let mut diagnostics = GitDiagnostics::default();
    while let Some(record) = records.next() {
        changed_files += 1;
        let code = record.as_bytes().get(..2).unwrap_or_default();
        if matches!(code, b"DD" | b"AU" | b"UD" | b"UA" | b"DU" | b"AA" | b"UU") {
            diagnostics.conflicts += 1;
        } else if code.contains(&b'D') {
            diagnostics.deleted_files += 1;
        }
        if record
            .as_bytes()
            .iter()
            .take(2)
            .any(|byte| matches!(byte, b'R' | b'C'))
        {
            records.next();
        }
    }
    diagnostics.large_deletion = diagnostics.deleted_files >= 10;
    Ok(Some(GitInfo {
        branch: branch.trim().to_owned(),
        changed_files,
        diagnostics,
    }))
}

pub fn change_status(changed_files: usize) -> String {
    if changed_files == 0 {
        "clean".to_owned()
    } else {
        format!("{changed_files} changed file(s)")
    }
}

pub fn commit_count(directory: &Path) -> Result<Option<usize>, CommandError> {
    match run_git(directory, &["rev-parse", "--verify", "--quiet", "HEAD"]) {
        Err(error) if error.kind == ErrorKind::Failed && error.exit_code == Some(1) => {
            return Ok(None);
        }
        Err(error) => return Err(error),
        Ok(_) => {}
    }
    let output = run_git(directory, &["rev-list", "--count", "HEAD"])?;
    output
        .trim()
        .parse()
        .map(Some)
        .map_err(|error| CommandError {
            kind: ErrorKind::Failed,
            command: "git rev-list --count HEAD".into(),
            message: format!("invalid commit count: {error}"),
            stdout: output,
            stderr: String::new(),
            exit_code: Some(0),
        })
}

pub fn latest_tag(directory: &Path) -> Result<Option<String>, CommandError> {
    match run_git(directory, &["describe", "--tags", "--abbrev=0"]) {
        Ok(value) => Ok(Some(value.trim().to_owned())),
        Err(error)
            if error.kind == ErrorKind::Failed
                && error.exit_code == Some(128)
                && (error.stderr.starts_with("fatal: No names found")
                    || error.stderr.starts_with("fatal: No tags can describe")) =>
        {
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

pub fn latest_commit(directory: &Path) -> Result<Option<CommitInfo>, CommandError> {
    match run_git(directory, &["rev-parse", "--verify", "--quiet", "HEAD"]) {
        Ok(hash) => {
            let message = run_git(directory, &["log", "-1", "--format=%s", "HEAD"])?;
            Ok(Some(CommitInfo {
                hash: hash.trim().to_owned(),
                message: message.trim().to_owned(),
            }))
        }
        Err(error) if error.kind == ErrorKind::Failed && error.exit_code == Some(1) => Ok(None),
        Err(error) => Err(error),
    }
}

fn run_git(directory: &Path, arguments: &[&str]) -> Result<String, CommandError> {
    process::run(
        Command::new("git")
            .args(arguments)
            .current_dir(directory)
            // Stable diagnostics for repository detection; no prompts or optional writes.
            .env("LC_ALL", "C")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_OPTIONAL_LOCKS", "0"),
        process::TIMEOUT,
    )
}

pub fn run_command(program: &str, arguments: &[&str]) -> Result<String, CommandError> {
    process::run(Command::new(program).args(arguments), process::TIMEOUT)
        .map(|value| value.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn git(directory: &Path, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(directory)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn recognizes_unborn_clean_dirty_and_detached_repositories() {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init", "-b", "main"]);
        let unborn = inspect(dir.path()).unwrap().unwrap();
        assert_eq!(unborn.branch, "main");
        assert_eq!(unborn.changed_files, 0);
        assert_eq!(commit_count(dir.path()).unwrap(), None);
        std::fs::write(dir.path().join("file.txt"), "one\n").unwrap();
        assert_eq!(inspect(dir.path()).unwrap().unwrap().changed_files, 1);
        git(dir.path(), &["add", "."]);
        git(
            dir.path(),
            &[
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-m",
                "initial",
            ],
        );
        assert_eq!(inspect(dir.path()).unwrap().unwrap().changed_files, 0);
        assert_eq!(commit_count(dir.path()).unwrap(), Some(1));
        assert_eq!(latest_tag(dir.path()).unwrap(), None);
        git(dir.path(), &["-c", "tag.gpgSign=false", "tag", "v1"]);
        assert_eq!(latest_tag(dir.path()).unwrap().as_deref(), Some("v1"));
        git(dir.path(), &["mv", "file.txt", "renamed.txt"]);
        assert_eq!(inspect(dir.path()).unwrap().unwrap().changed_files, 1);
        git(dir.path(), &["checkout", "--detach"]);
        assert!(!inspect(dir.path()).unwrap().unwrap().branch.is_empty());
    }

    #[test]
    fn recognizes_non_repository() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(inspect(dir.path()).unwrap(), None);
    }

    #[test]
    fn missing_commands_and_invalid_directories_are_errors() {
        assert_eq!(
            run_command("yoo-command-that-does-not-exist", &[])
                .unwrap_err()
                .kind,
            ErrorKind::Io
        );
        let dir = tempfile::tempdir().unwrap();
        assert!(inspect(&dir.path().join("missing")).is_err());
    }

    #[test]
    fn failures_and_timeouts_are_not_clean_or_non_repositories() {
        for kind in [ErrorKind::Failed, ErrorKind::Timeout] {
            for failing_command in ["rev-parse", "symbolic-ref", "status"] {
                let expected = CommandError {
                    kind,
                    command: "git".into(),
                    message: "test failure".into(),
                    stdout: String::new(),
                    stderr: "broken".into(),
                    exit_code: Some(128),
                };
                let result = inspect_with(|args| {
                    if args[0] == failing_command {
                        Err(expected.clone())
                    } else {
                        Ok("main".into())
                    }
                });
                assert_eq!(result.unwrap_err(), expected);
            }
        }
    }

    #[test]
    fn counts_paths_with_newlines_and_rename_records() {
        let report = inspect_with(|args| {
            Ok(match args[0] {
                "status" => "R  new\nname\0old\nname\0?? other\nfile\0",
                _ => "main",
            }
            .into())
        })
        .unwrap()
        .unwrap();
        assert_eq!(report.changed_files, 2);
    }

    #[test]
    fn distinguishes_conflicts_deletions_and_renames() {
        let status = format!(
            "UU conflict\0R  renamed\0old\0?? untracked\0{}",
            (0..10).map(|i| format!(" D file{i}\0")).collect::<String>()
        );
        let info = inspect_with(|args| {
            Ok(if args[0] == "status" {
                status.clone()
            } else {
                "main".into()
            })
        })
        .unwrap()
        .unwrap();
        assert_eq!(info.diagnostics.conflicts, 1);
        assert_eq!(info.diagnostics.deleted_files, 10);
        assert!(info.diagnostics.large_deletion);
        assert_eq!(info.changed_files, 13);
    }

    #[test]
    fn reports_upstream_divergence_and_worktree_operation() {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init", "-b", "main"]);
        std::fs::write(dir.path().join("file"), "one").unwrap();
        git(dir.path(), &["add", "."]);
        let commit = |message| {
            git(
                dir.path(),
                &[
                    "-c",
                    "user.name=Test",
                    "-c",
                    "user.email=test@example.invalid",
                    "-c",
                    "commit.gpgsign=false",
                    "commit",
                    "--allow-empty",
                    "-m",
                    message,
                ],
            )
        };
        commit("initial");
        git(dir.path(), &["branch", "upstream"]);
        git(dir.path(), &["branch", "--set-upstream-to=upstream"]);
        commit("local");
        let info = inspect(dir.path()).unwrap().unwrap();
        assert_eq!(info.diagnostics.upstream.as_deref(), Some("upstream"));
        assert_eq!(info.diagnostics.ahead, 1);
        assert_eq!(info.diagnostics.behind, 0);
        git(dir.path(), &["checkout", "upstream"]);
        commit("remote");
        git(dir.path(), &["checkout", "main"]);
        assert_eq!(inspect(dir.path()).unwrap().unwrap().diagnostics.behind, 1);
        let worktree = dir.path().join("linked");
        git(
            dir.path(),
            &["worktree", "add", "--detach", worktree.to_str().unwrap()],
        );
        let path = run_git(&worktree, &["rev-parse", "--git-path", "rebase-merge"]).unwrap();
        std::fs::create_dir(worktree.join(path.trim())).unwrap();
        assert_eq!(
            inspect(&worktree)
                .unwrap()
                .unwrap()
                .diagnostics
                .operation
                .as_deref(),
            Some("rebase")
        );
    }

    #[test]
    fn formats_change_status() {
        assert_eq!(change_status(0), "clean");
        assert_eq!(change_status(3), "3 changed file(s)");
    }
}
