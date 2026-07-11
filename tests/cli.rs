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
fn version_reports_the_package_version() {
    let output = yoo(&std::env::temp_dir(), &["--version"]);

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        format!("yoo {}", env!("CARGO_PKG_VERSION"))
    );
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
fn project_json_reads_workspace_inherited_cargo_metadata() {
    let directory = temporary_directory("cargo");
    fs::create_dir_all(directory.join("src")).expect("source directory should be created");
    fs::write(
        directory.join("Cargo.toml"),
        r#"
[workspace.package]
version = "7.2.0"
edition = "2024"
license = "MIT"

[package]
name = 'workspace-demo'
version.workspace = true
edition.workspace = true
license.workspace = true
"#,
    )
    .expect("manifest should be written");
    fs::write(directory.join("src/main.rs"), "fn main() {}\n").expect("source should be written");

    let output = yoo(&directory, &["project", "--json"]);
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("project output should be JSON");

    assert!(output.status.success());
    assert_eq!(report["project"]["name"], "workspace-demo");
    assert_eq!(report["project"]["version"], "7.2.0");
    assert_eq!(report["project"]["edition"], "2024");
    assert_eq!(report["project"]["license"], "MIT");
    assert_eq!(report["source"]["files"], 1);

    fs::remove_dir_all(directory).expect("test directory should be removed");
}

#[test]
fn fetch_json_reads_python_project_metadata() {
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
    assert_eq!(report["project"]["name"], "python-demo");
    assert_eq!(report["project"]["kind"], "Python");
    assert_eq!(report["project"]["version"], "4.3.2");

    fs::remove_dir_all(directory).expect("test directory should be removed");
}
