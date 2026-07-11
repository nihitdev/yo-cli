use std::{
    io::Read,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(5);

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
    run_command_in(directory, "git", arguments)
}

pub fn run_command(program: &str, arguments: &[&str]) -> Option<String> {
    command_stdout(Command::new(program).args(arguments), COMMAND_TIMEOUT)
}

pub fn run_command_in(directory: &Path, program: &str, arguments: &[&str]) -> Option<String> {
    command_stdout(
        Command::new(program).args(arguments).current_dir(directory),
        COMMAND_TIMEOUT,
    )
}

fn command_stdout(command: &mut Command, timeout: Duration) -> Option<String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdout = child.stdout.take()?;
    let output_reader = thread::spawn(move || {
        let mut output = Vec::new();
        stdout.read_to_end(&mut output).map(|_| output)
    });
    let deadline = Instant::now() + timeout;

    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) | Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = output_reader.join();
                return None;
            }
        }
    };
    let output = output_reader.join().ok()?.ok()?;

    if !status.success() {
        return None;
    }

    String::from_utf8(output)
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

    #[test]
    fn missing_commands_return_none() {
        assert_eq!(
            run_command("yoo-command-that-does-not-exist", &["--version"]),
            None
        );
    }
}
