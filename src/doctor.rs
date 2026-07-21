use std::path::Path;

use crate::{config, fetch, git};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub label: &'static str,
    pub status: Status,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub checks: Vec<Check>,
}

impl Report {
    pub fn pass_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == Status::Pass)
            .count()
    }

    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == Status::Warn)
            .count()
    }

    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status == Status::Fail)
            .count()
    }
}

pub fn collect(directory: &Path) -> Report {
    let mut checks = vec![
        command_check("Rust compiler", "rustc", &["--version"]),
        command_check("Cargo", "cargo", &["--version"]),
        command_check("Git", "git", &["--version"]),
        command_check("Rustfmt", "rustfmt", &["--version"]),
        command_check("Clippy", "cargo", &["clippy", "--version"]),
    ];

    checks.push(config_check());
    checks.push(project_check(directory));
    checks.push(repository_check(directory));

    Report { checks }
}

pub fn print(report: &Report) {
    println!("🩺 yoo doctor\n");

    for check in &report.checks {
        let icon = match check.status {
            Status::Pass => "✔",
            Status::Warn => "!",
            Status::Fail => "✘",
        };
        println!("{icon} {:<15} {}", check.label, check.detail);
    }

    println!();
    println!(
        "Health: {} passed, {} warning(s), {} failed",
        report.pass_count(),
        report.warn_count(),
        report.fail_count()
    );

    if report.fail_count() == 0 && report.warn_count() == 0 {
        println!("Everything looks good. 🚀");
    } else if report.fail_count() == 0 {
        println!("Usable setup, with a few optional things to improve.");
    } else {
        println!("Fix the failed checks above, then run `yoo doctor` again.");
    }
}

fn command_check(label: &'static str, program: &str, arguments: &[&str]) -> Check {
    match git::run_command(program, arguments) {
        Some(version) => Check {
            label,
            status: Status::Pass,
            detail: version,
        },
        None => Check {
            label,
            status: Status::Fail,
            detail: "not found in PATH".to_owned(),
        },
    }
}

fn config_check() -> Check {
    let path = config::config_path();

    if !path.exists() {
        return Check {
            label: "Yoo config",
            status: Status::Warn,
            detail: format!(
                "not found; defaults active — run `yoo init` ({})",
                path.display()
            ),
        };
    }

    match config::load() {
        Ok(_) => Check {
            label: "Yoo config",
            status: Status::Pass,
            detail: format!("valid ({})", path.display()),
        },
        Err(error) => Check {
            label: "Yoo config",
            status: Status::Fail,
            detail: error.to_string(),
        },
    }
}

fn project_check(directory: &Path) -> Check {
    let project = fetch::detect_project(directory);

    if let Some(manifest) = project.manifest {
        Check {
            label: "Project",
            status: Status::Pass,
            detail: format!("{} detected ({manifest})", project.kind),
        }
    } else {
        Check {
            label: "Project",
            status: Status::Warn,
            detail: "no supported project manifest found in this directory".to_owned(),
        }
    }
}

fn repository_check(directory: &Path) -> Check {
    match git::inspect(directory) {
        Some(info) => Check {
            label: "Git repository",
            status: Status::Pass,
            detail: format!(
                "branch `{}`; {} changed file(s)",
                info.branch, info.changed_files
            ),
        },
        None => Check {
            label: "Git repository",
            status: Status::Warn,
            detail: "not a Git repository".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_counts_statuses() {
        let report = Report {
            checks: vec![
                Check {
                    label: "A",
                    status: Status::Pass,
                    detail: String::new(),
                },
                Check {
                    label: "B",
                    status: Status::Warn,
                    detail: String::new(),
                },
                Check {
                    label: "C",
                    status: Status::Fail,
                    detail: String::new(),
                },
            ],
        };

        assert_eq!(report.pass_count(), 1);
        assert_eq!(report.warn_count(), 1);
        assert_eq!(report.fail_count(), 1);
    }

    #[test]
    fn project_check_detects_non_rust_projects() {
        let directory =
            std::env::temp_dir().join(format!("yoo-doctor-node-{}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("test directory should be created");
        std::fs::write(directory.join("package.json"), "{}")
            .expect("package manifest should be written");

        let check = project_check(&directory);

        assert_eq!(check.label, "Project");
        assert_eq!(check.status, Status::Pass);
        assert_eq!(check.detail, "Node.js detected (package.json)");

        std::fs::remove_dir_all(directory).expect("test directory should be removed");
    }
}
