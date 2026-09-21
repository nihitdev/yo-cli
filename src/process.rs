//! Short-lived, non-interactive commands. Capture into anonymous files instead of
//! pipes: an inherited output handle must never make a reader or join block.
use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[path = "process_tree.rs"]
mod tree;

pub const TIMEOUT: Duration = Duration::from_secs(5);
const OUTPUT_LIMIT: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Io,
    Failed,
    Timeout,
    OutputLimit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandError {
    pub kind: ErrorKind,
    pub command: String,
    pub message: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.command, self.message)?;
        if !self.stderr.trim().is_empty() {
            write!(f, ": {}", self.stderr.trim())?;
        }
        Ok(())
    }
}
impl std::error::Error for CommandError {}

pub fn run(command: &mut Command, timeout: Duration) -> Result<String, CommandError> {
    let description = format!(
        "{} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );
    let io_error = |error: io::Error| CommandError {
        kind: ErrorKind::Io,
        command: description.clone(),
        message: error.to_string(),
        stdout: String::new(),
        stderr: String::new(),
        exit_code: None,
    };
    let mut stdout = tempfile::tempfile().map_err(io_error)?;
    let mut stderr = tempfile::tempfile().map_err(io_error)?;
    command
        .stdin(Stdio::null())
        .stdout(stdout.try_clone().map_err(io_error)?)
        .stderr(stderr.try_clone().map_err(io_error)?);
    let mut child = tree::ProcessTree::spawn(command).map_err(io_error)?;
    // Release the parent's duplicated output handles immediately.
    command.stdout(Stdio::null()).stderr(Stdio::null());
    let deadline = Instant::now() + timeout;
    let outcome = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Err(error) => break Err((ErrorKind::Io, error.to_string())),
            Ok(None) => {}
        }
        if Instant::now() >= deadline {
            break Err((
                ErrorKind::Timeout,
                format!("timed out after {} ms", timeout.as_millis()),
            ));
        }
        match stdout
            .metadata()
            .and_then(|out| stderr.metadata().map(|err| (out.len(), err.len())))
        {
            Ok((out, err)) if out > OUTPUT_LIMIT || err > OUTPUT_LIMIT => {
                break Err((
                    ErrorKind::OutputLimit,
                    "output exceeded 8 MiB per stream".into(),
                ));
            }
            Err(error) => break Err((ErrorKind::Io, error.to_string())),
            _ => {}
        }
        thread::sleep(
            Duration::from_millis(10).min(deadline.saturating_duration_since(Instant::now())),
        );
    };
    // Also stop descendants after a normal exit. No wait here depends on EOF.
    let cleanup = child.finish();
    let out = capture(&mut stdout).map_err(io_error)?;
    let err = capture(&mut stderr).map_err(io_error)?;
    let exit_code = outcome.as_ref().ok().and_then(|status| status.code());
    let failure = match outcome {
        Err(error) => Some(error),
        Ok(status) if !status.success() => {
            Some((ErrorKind::Failed, format!("exited with {status}")))
        }
        Ok(_) if out.1 || err.1 => Some((
            ErrorKind::OutputLimit,
            "output exceeded 8 MiB per stream".into(),
        )),
        // Once the command itself completed successfully, process-group cleanup
        // is best-effort. Restricted environments may reject signalling the
        // group even though the leader exited successfully.
        Ok(_) => None,
    };
    if let Some((kind, mut message)) = failure {
        if kind != ErrorKind::Io {
            if let Err(error) = cleanup {
                message.push_str(&format!("; process cleanup failed: {error}"));
            }
        }
        return Err(CommandError {
            kind,
            command: description,
            message,
            stdout: out.0,
            stderr: err.0,
            exit_code,
        });
    }
    Ok(out.0)
}

fn capture(file: &mut File) -> io::Result<(String, bool)> {
    // Snapshot the length so even an escaped process cannot prolong the read.
    let length = file.metadata()?.len();
    file.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    file.take(length.min(OUTPUT_LIMIT))
        .read_to_end(&mut bytes)?;
    Ok((
        String::from_utf8_lossy(&bytes).into_owned(),
        length > OUTPUT_LIMIT,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[cfg(unix)]
    #[test]
    fn preserves_stdout_and_stderr_on_failure() {
        let error = run(
            Command::new("sh").args(["-c", "printf output; printf diagnostic >&2; exit 7"]),
            TIMEOUT,
        )
        .unwrap_err();
        assert_eq!(error.kind, ErrorKind::Failed);
        assert_eq!(error.stdout, "output");
        assert_eq!(error.stderr, "diagnostic");
        assert_eq!(error.exit_code, Some(7));
    }

    #[cfg(unix)]
    fn descendant_case(parent_waits: bool) {
        // Both pipes used to be inherited here. The descendant outlives its
        // parent (or holds both output handles throughout a parent timeout).
        let script = if parent_waits {
            "sleep 30 & printf '%s %s' $$ $!; printf diagnostic >&2; wait"
        } else {
            "sleep 30 & printf '%s %s' $$ $!; printf diagnostic >&2; exit 0"
        };
        let started = Instant::now();
        let result = run(
            Command::new("sh").args(["-c", script]),
            Duration::from_millis(200),
        );
        assert!(started.elapsed() < Duration::from_secs(3));
        let output = if parent_waits {
            let error = result.unwrap_err();
            assert_eq!(error.kind, ErrorKind::Timeout);
            assert_eq!(error.stderr, "diagnostic");
            error.stdout
        } else {
            result.unwrap()
        };
        let pids: Vec<i32> = output
            .split_whitespace()
            .map(|pid| pid.parse().unwrap())
            .collect();
        assert_eq!(pids.len(), 2);
        // The direct child is reaped on all Unix platforms; on Linux our
        // subreaper also guarantees no orphaned descendant zombies remain.
        let count = if cfg!(target_os = "linux") { 2 } else { 1 };
        for pid in pids.iter().take(count) {
            assert_eq!(
                unsafe { libc::kill(*pid, 0) },
                -1,
                "process {pid} survived cleanup"
            );
            assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
        }
    }

    #[cfg(unix)]
    #[test]
    fn timeout_kills_and_reaps_child_and_inherited_output_descendant() {
        descendant_case(true);
    }

    #[cfg(unix)]
    #[test]
    fn normal_exit_does_not_wait_for_inherited_output_descendant() {
        descendant_case(false);
    }

    #[cfg(unix)]
    #[test]
    fn stdin_is_closed_and_large_outputs_are_bounded() {
        assert_eq!(
            run(Command::new("sh").args(["-c", "cat; printf done"]), TIMEOUT).unwrap(),
            "done"
        );
        let error = run(
            Command::new("sh").args(["-c", "head -c 9000000 /dev/zero"]),
            TIMEOUT,
        )
        .unwrap_err();
        assert_eq!(error.kind, ErrorKind::OutputLimit);
        assert_eq!(error.stdout.len(), OUTPUT_LIMIT as usize);
    }

    #[cfg(windows)]
    #[test]
    fn windows_timeout_cleans_job_with_inherited_handles() {
        let start = Instant::now();
        let error = run(
            Command::new("cmd")
                .args(["/C", "start /B ping -n 30 127.0.0.1 & ping -n 30 127.0.0.1"]),
            Duration::from_millis(200),
        )
        .unwrap_err();
        assert_eq!(error.kind, ErrorKind::Timeout);
        assert!(start.elapsed() < Duration::from_secs(3));
    }

    #[cfg(windows)]
    #[test]
    fn windows_normal_exit_cleans_job_with_inherited_handles() {
        let start = Instant::now();
        run(
            Command::new("cmd").args(["/C", "start /B ping -n 30 127.0.0.1 & exit /B 0"]),
            Duration::from_secs(2),
        )
        .unwrap();
        assert!(start.elapsed() < Duration::from_secs(3));
    }
}
