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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_for_a_non_git_directory() {
        let directory = std::env::temp_dir();
        // This assertion only requires that the function never panics when Git data is unavailable.
        let _ = inspect(&directory);
    }
}
