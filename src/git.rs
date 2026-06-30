use std::{
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitInfo {
    pub branch: String,
    pub changed_files: usize,
}

pub fn inspect(directory: &Path) -> Option<GitInfo> {
    let branch = run_git(directory, &["rev-parse", "--abbrev-ref", "HEAD"])?;

    if branch.is_empty() {
        return None;
    }

    let changed_files = run_git(directory, &["status", "--porcelain"])
        .map(|status| status.lines().count())
        .unwrap_or(0);

    Some(GitInfo {
        branch,
        changed_files,
    })
}

pub fn commit_count(directory: &Path) -> Option<usize> {
    run_git(directory, &["rev-list", "--count", "HEAD"])
        .and_then(|value| value.parse::<usize>().ok())
}

pub fn latest_tag(directory: &Path) -> Option<String> {
    run_git(directory, &["describe", "--tags", "--abbrev=0"])
}

fn run_git(directory: &Path, arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(directory)
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout)
        .ok()
        .map(|text| text.trim().to_owned())
}

pub fn run_command(program: &str, arguments: &[&str]) -> Option<String> {
    let output = Command::new(program)
        .args(arguments)
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout)
        .ok()
        .map(|text| text.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_never_panics_for_a_temp_directory() {
        let directory = std::env::temp_dir();
        let _ = inspect(&directory);
    }

    #[test]
    fn git_helpers_never_panic_for_a_temp_directory() {
        let directory = std::env::temp_dir();
        let _ = commit_count(&directory);
        let _ = latest_tag(&directory);
    }
}
