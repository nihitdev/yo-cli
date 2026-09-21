use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn yoo(directory: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_yoo"))
        .args(arguments)
        .current_dir(directory)
        .output()
        .expect("yoo should run")
}

fn temporary_directory(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX epoch")
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "yoo-cli-test-{name}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("test directory should be created");
    directory
}

#[test]
fn version_command_and_flag_report_the_package_version() {
    for arguments in [["version"], ["--version"], ["-V"]] {
        let output = yoo(&std::env::temp_dir(), &arguments);

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            format!("yoo {}", env!("CARGO_PKG_VERSION"))
        );
    }
}

#[test]
fn readme_version_example_matches_the_package_version() {
    let readme = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md"))
        .expect("README should be readable");

    assert!(readme.contains(&format!("yoo {}", env!("CARGO_PKG_VERSION"))));
}

#[test]
fn invalid_arguments_exit_with_usage_error() {
    let output = yoo(&std::env::temp_dir(), &["project", "--unknown"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2));
    assert!(stderr.contains("unknown project option"));
    assert!(stderr.contains("USAGE:"));
}

#[test]
fn edit_reports_when_the_requested_editor_cannot_be_launched() {
    let output = yoo(
        &std::env::temp_dir(),
        &["edit", "--editor", "yoo-editor-that-does-not-exist"],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("could not launch editor `yoo-editor-that-does-not-exist`"));
}

#[test]
fn completion_commands_generate_shell_scripts() {
    for (shell, marker) in [
        ("bash", "complete -F _yoo yoo"),
        ("zsh", "#compdef yoo"),
        ("fish", "complete -c yoo"),
        ("powershell", "Register-ArgumentCompleter"),
    ] {
        let output = yoo(&std::env::temp_dir(), &["completions", shell]);
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains(marker));
    }
}

#[test]
fn redirected_display_output_does_not_contain_ansi_codes() {
    let output = yoo(&std::env::temp_dir(), &["fetch", "--no-art"]);

    assert!(output.status.success());
    assert!(!output.stdout.windows(2).any(|bytes| bytes == b"\x1b["));
}

#[test]
fn project_json_reads_cargo_package_metadata() {
    let directory = temporary_directory("cargo");
    fs::create_dir_all(directory.join("src")).expect("source directory should be created");
    fs::write(
        directory.join("Cargo.toml"),
        r#"
[workspace.package]
name = "wrong-section"

[package]
name = 'lean-demo'
version = "7.2.0" # inline comments are allowed
edition = "2024"
license = "MIT"
"#,
    )
    .expect("manifest should be written");
    fs::write(directory.join("src/main.rs"), "fn main() {}\n").expect("source should be written");

    let output = yoo(&directory, &["project", "--json"]);
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("project output should be JSON");

    assert!(output.status.success());
    assert_eq!(report["project"]["name"], "lean-demo");
    assert_eq!(report["project"]["version"], "7.2.0");
    assert_eq!(report["project"]["edition"], "2024");
    assert_eq!(report["project"]["license"], "MIT");
    assert_eq!(report["source"]["files"], 1);

    fs::remove_dir_all(directory).expect("test directory should be removed");
}

#[test]
fn fetch_json_detects_python_without_parsing_extra_metadata() {
    let directory = temporary_directory("python");
    fs::write(
        directory.join("pyproject.toml"),
        r#"
[project]
name = "python-demo"
version = "4.3.2"
"#,
    )
    .expect("manifest should be written");

    let output = yoo(&directory, &["fetch", "--json"]);
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("fetch output should be JSON");

    assert!(output.status.success());
    assert!(
        report["project"]["name"]
            .as_str()
            .is_some_and(|name| name.starts_with("yoo-cli-test-python-"))
    );
    assert_eq!(report["project"]["kind"], "Python");
    assert!(report["project"]["version"].is_null());

    fs::remove_dir_all(directory).expect("test directory should be removed");
}

#[cfg(unix)]
fn fake_tool(directory: &Path, name: &str, script: &str) {
    use std::os::unix::fs::PermissionsExt;
    let path = directory.join(name);
    fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(unix)]
#[test]
fn doctor_exit_status_distinguishes_failures_from_advisory_warnings() {
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    fs::create_dir(&bin).unwrap();
    for tool in ["rustc", "cargo", "rustfmt"] {
        fake_tool(&bin, tool, "echo 'tool 1.0'");
    }
    fake_tool(
        &bin,
        "git",
        "if [ \"$1\" = rev-parse ]; then echo 'fatal: not a git repository (or any of the parent directories): .git' >&2; exit 128; fi; echo 'git 1.0'",
    );
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_yoo"))
            .arg("doctor")
            .current_dir(root.path())
            .env("PATH", &bin)
            .env("HOME", root.path())
            .env("USERPROFILE", root.path())
            .env("XDG_CONFIG_HOME", root.path())
            .output()
            .unwrap()
    };
    let success = run();
    assert!(
        success.status.success(),
        "{}",
        String::from_utf8_lossy(&success.stderr)
    );
    assert!(String::from_utf8_lossy(&success.stdout).contains("0 failed"));
    fake_tool(&bin, "rustc", "echo 'compiler broken' >&2; exit 7");
    let failed = run();
    assert_eq!(failed.status.code(), Some(1));
    let report = String::from_utf8_lossy(&failed.stdout);
    assert!(report.contains("1 failed"));
    assert!(report.contains("compiler broken"));
    assert!(String::from_utf8_lossy(&failed.stderr).contains("doctor check(s) failed"));
}

#[cfg(unix)]
#[test]
fn git_failure_in_json_mode_is_an_error_not_a_clean_report() {
    let root = tempfile::tempdir().unwrap();
    fake_tool(root.path(), "git", "echo 'git is broken' >&2; exit 128");
    for command in ["project", "fetch", "status"] {
        let output = Command::new(env!("CARGO_BIN_EXE_yoo"))
            .args([command, "--json"])
            .env("PATH", root.path())
            .current_dir(root.path())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("git is broken"));
    }
}

#[cfg(unix)]
#[test]
fn git_status_timeout_is_reported_and_exits_nonzero() {
    let root = tempfile::tempdir().unwrap();
    fake_tool(
        root.path(),
        "git",
        "case \"$1\" in rev-parse) echo .git;; symbolic-ref) echo main;; status) /bin/sleep 30;; esac",
    );
    let start = std::time::Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_yoo"))
        .args(["project", "--json"])
        .env("PATH", root.path())
        .current_dir(root.path())
        .output()
        .unwrap();
    assert!(start.elapsed() < std::time::Duration::from_secs(10));
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("git status"));
    assert!(error.contains("timed out"));
}
