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
    for arguments in [["version"], ["--version"]] {
        let output = yoo(&std::env::temp_dir(), &arguments);

        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            format!("yoo {}", env!("CARGO_PKG_VERSION"))
        );
    }
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
