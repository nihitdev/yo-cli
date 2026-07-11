use std::path::Path;

use crate::{config, fetch, git, project};

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
    let detected = fetch::detect_project(directory);
    let mut checks = language_checks(directory, &detected.kind);

    checks.push(command_check("Git", "git", &["--version"]));
    checks.push(config_check());
    checks.push(project_check(&detected));
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

fn command_check_any(label: &'static str, commands: &[(&str, &[&str])]) -> Check {
    for (program, arguments) in commands {
        if let Some(version) = git::run_command(program, arguments) {
            return Check {
                label,
                status: Status::Pass,
                detail: version,
            };
        }
    }

    Check {
        label,
        status: Status::Fail,
        detail: "not found in PATH".to_owned(),
    }
}

fn language_checks(directory: &Path, language: &str) -> Vec<Check> {
    match language {
        "Rust" => vec![
            command_check("Rust compiler", "rustc", &["--version"]),
            command_check("Cargo", "cargo", &["--version"]),
            command_check("Rustfmt", "rustfmt", &["--version"]),
            command_check("Clippy", "cargo", &["clippy", "--version"]),
        ],
        "Node.js" => {
            let mut checks = vec![command_check("Node.js", "node", &["--version"])];
            let package_manager = project::package_manager(directory, language);
            let manager_check = match package_manager.as_deref() {
                Some("pnpm") => command_check("pnpm", "pnpm", &["--version"]),
                Some("Yarn") => command_check("Yarn", "yarn", &["--version"]),
                Some("Bun") => command_check("Bun", "bun", &["--version"]),
                _ => command_check("npm", "npm", &["--version"]),
            };
            checks.push(manager_check);
            checks
        }
        "Python" => {
            let mut checks = vec![command_check_any(
                "Python",
                &[
                    ("python3", &["--version"]),
                    ("python", &["--version"]),
                    ("py", &["--version"]),
                ],
            )];
            let package_manager = project::package_manager(directory, language);
            let manager_check = match package_manager.as_deref() {
                Some("uv") => command_check("uv", "uv", &["--version"]),
                Some("Poetry") => command_check("Poetry", "poetry", &["--version"]),
                Some("Pipenv") => command_check("Pipenv", "pipenv", &["--version"]),
                _ => command_check_any("pip", &[("pip3", &["--version"]), ("pip", &["--version"])]),
            };
            checks.push(manager_check);
            checks
        }
        "Go" => vec![command_check("Go", "go", &["version"])],
        "Java" => {
            let mut checks = vec![command_check("Java", "java", &["--version"])];
            let manager_check = if directory.join("pom.xml").is_file() {
                command_check("Maven", "mvn", &["--version"])
            } else {
                command_check("Gradle", "gradle", &["--version"])
            };
            checks.push(manager_check);
            checks
        }
        ".NET" => vec![command_check(".NET SDK", "dotnet", &["--version"])],
        _ => Vec::new(),
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

fn project_check(project: &fetch::ProjectInfo) -> Check {
    if let Some(manifest) = project.manifest.as_deref() {
        Check {
            label: "Current project",
            status: Status::Pass,
            detail: format!("{} ({manifest})", project.kind),
        }
    } else {
        Check {
            label: "Current project",
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
    fn project_check_describes_the_detected_language() {
        let project = fetch::ProjectInfo {
            name: "demo".to_owned(),
            kind: "Python".to_owned(),
            manifest: Some("pyproject.toml".to_owned()),
            version: Some("1.0.0".to_owned()),
            directory: ".".to_owned(),
        };

        let check = project_check(&project);

        assert_eq!(check.status, Status::Pass);
        assert_eq!(check.detail, "Python (pyproject.toml)");
    }
}
