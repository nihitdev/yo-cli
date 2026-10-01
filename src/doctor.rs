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
    /// Warnings are advisory; every failed check makes doctor unsuitable as a CI gate.
    pub fn result(&self) -> std::io::Result<()> {
        if self.fail_count() == 0 {
            Ok(())
        } else {
            Err(std::io::Error::other(format!(
                "{} doctor check(s) failed",
                self.fail_count()
            )))
        }
    }

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
    let mut checks = vec![command_check("Git", "git", &["--version"])];
    for (label, program, arguments) in project_tools(directory) {
        checks.push(command_check(label, program, &arguments));
    }

    checks.push(config_check());
    checks.push(project_check(directory));
    checks.push(repository_check(directory));

    Report { checks }
}

type ToolCheck = (&'static str, &'static str, Vec<&'static str>);

fn project_tools(directory: &Path) -> Vec<ToolCheck> {
    match fetch::detect_project(directory).kind.as_str() {
        "Rust" => vec![
            ("Rust compiler", "rustc", vec!["--version"]),
            ("Cargo", "cargo", vec!["--version"]),
            ("Rustfmt", "rustfmt", vec!["--version"]),
            ("Clippy", "cargo", vec!["clippy", "--version"]),
        ],
        "Node.js" => {
            let manager = if directory.join("pnpm-lock.yaml").is_file() {
                "pnpm"
            } else if directory.join("yarn.lock").is_file() {
                "yarn"
            } else if directory.join("bun.lock").is_file() || directory.join("bun.lockb").is_file()
            {
                "bun"
            } else {
                "npm"
            };
            vec![
                ("Node.js", "node", vec!["--version"]),
                ("Package manager", manager, vec!["--version"]),
            ]
        }
        "Python" => vec![(
            "Python",
            if cfg!(windows) { "python" } else { "python3" },
            vec!["--version"],
        )],
        "Go" => vec![("Go", "go", vec!["version"])],
        "Java" => vec![("Java", "java", vec!["--version"])],
        ".NET" => vec![(".NET SDK", "dotnet", vec!["--version"])],
        _ => Vec::new(),
    }
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
        Ok(version) => Check {
            label,
            status: Status::Pass,
            detail: version,
        },
        Err(error) => Check {
            label,
            status: Status::Fail,
            detail: error.to_string(),
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
        Ok(Some(info)) => {
            let diagnostics = &info.diagnostics;
            Check {
                label: "Git repository",
                status: if diagnostics.conflicts > 0
                    || diagnostics.large_deletion
                    || diagnostics.operation.is_some()
                {
                    Status::Warn
                } else {
                    Status::Pass
                },
                detail: format!(
                    "branch `{}`; {}; {}",
                    info.branch,
                    git::change_status(info.changed_files),
                    diagnostics.summary()
                )
                .trim_end_matches("; ")
                .to_owned(),
            }
        }
        Ok(None) => Check {
            label: "Git repository",
            status: Status::Warn,
            detail: "not a Git repository".to_owned(),
        },
        Err(error) => Check {
            label: "Git repository",
            status: Status::Fail,
            detail: error.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_follow_project_manifests_and_lockfiles() {
        let dir = tempfile::tempdir().unwrap();
        assert!(project_tools(dir.path()).is_empty());
        for (manifest, expected) in [
            ("package.json", "node"),
            (
                "pyproject.toml",
                if cfg!(windows) { "python" } else { "python3" },
            ),
            ("go.mod", "go"),
            ("pom.xml", "java"),
            ("demo.csproj", "dotnet"),
            ("Cargo.toml", "rustc"),
        ] {
            std::fs::write(dir.path().join(manifest), "{}").unwrap();
            let tools = project_tools(dir.path());
            assert_eq!(tools[0].1, expected);
            if manifest == "package.json" {
                assert!(!tools.iter().any(|(_, tool, _)| *tool == "rustc"));
                std::fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();
                assert_eq!(project_tools(dir.path())[1].1, "pnpm");
            }
            std::fs::remove_file(dir.path().join(manifest)).unwrap();
        }
    }

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
    fn only_failed_checks_make_doctor_fail() {
        for status in [Status::Pass, Status::Warn, Status::Fail] {
            let report = Report {
                checks: vec![Check {
                    label: "test",
                    status,
                    detail: String::new(),
                }],
            };
            assert_eq!(report.result().is_err(), status == Status::Fail);
        }
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
