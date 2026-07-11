use std::{fs, io, path::Path};

use serde::Serialize;

use crate::{fetch, git, ui::Ui};

#[derive(Debug, Clone, Serialize)]
pub struct ProjectReport {
    pub yoo_version: String,
    pub project: ProjectDetails,
    pub source: SourceSummary,
    pub git: Option<GitProjectSummary>,
    pub files: ProjectFiles,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectDetails {
    pub name: String,
    pub language: String,
    pub package_manager: Option<String>,
    pub manifest: Option<String>,
    pub version: Option<String>,
    pub edition: Option<String>,
    pub license: Option<String>,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceSummary {
    pub files: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitProjectSummary {
    pub branch: String,
    pub changed_files: usize,
    pub commits: Option<usize>,
    pub latest_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectFiles {
    pub readme: bool,
    pub license: bool,
    pub changelog: bool,
    pub gitignore: bool,
    pub ci: bool,
}

pub fn collect(directory: &Path) -> ProjectReport {
    let detected = fetch::detect_project(directory);

    let manifest_contents = detected
        .manifest
        .as_deref()
        .and_then(|manifest| fs::read_to_string(directory.join(manifest)).ok());

    let edition = if detected.kind == "Rust" {
        manifest_contents
            .as_deref()
            .and_then(|contents| fetch::find_toml_string(contents, "edition"))
    } else {
        None
    };

    let license = if detected.kind == "Rust" {
        manifest_contents
            .as_deref()
            .and_then(|contents| fetch::find_toml_string(contents, "license"))
            .or_else(|| find_license_file(directory))
    } else {
        find_license_file(directory)
    };

    let git = git::inspect(directory).map(|info| GitProjectSummary {
        branch: info.branch,
        changed_files: info.changed_files,
        commits: git::commit_count(directory),
        latest_tag: git::latest_tag(directory),
    });

    ProjectReport {
        yoo_version: env!("CARGO_PKG_VERSION").to_owned(),
        project: ProjectDetails {
            name: detected.name,
            language: detected.kind.clone(),
            package_manager: package_manager(directory, &detected.kind),
            manifest: detected.manifest,
            version: detected.version,
            edition,
            license,
            directory: detected.directory,
        },
        source: count_source(directory, &detected.kind),
        git,
        files: ProjectFiles {
            readme: any_file_exists(directory, &["README.md", "README", "readme.md"]),
            license: any_file_exists(
                directory,
                &["LICENSE", "LICENSE.md", "LICENCE", "LICENCE.md"],
            ),
            changelog: any_file_exists(directory, &["CHANGELOG.md", "CHANGELOG", "HISTORY.md"]),
            gitignore: directory.join(".gitignore").is_file(),
            ci: has_ci_workflow(directory),
        },
    }
}

pub fn to_json(report: &ProjectReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

pub fn print(report: &ProjectReport, ui: &Ui) -> io::Result<()> {
    ui.heading("yoo project — project overview")?;
    ui.blank_line()?;

    ui.divider()?;
    ui.info("📦", "Name:", &report.project.name)?;
    ui.info("🔧", "Language:", &report.project.language)?;

    if let Some(package_manager) = report.project.package_manager.as_deref() {
        ui.info("📦", "Package manager:", package_manager)?;
    }

    if let Some(manifest) = report.project.manifest.as_deref() {
        ui.info("📄", "Manifest:", manifest)?;
    }

    if let Some(version) = report.project.version.as_deref() {
        ui.info("🏷", "Version:", version)?;
    }

    if let Some(edition) = report.project.edition.as_deref() {
        ui.info("🦀", "Edition:", edition)?;
    }

    if let Some(license) = report.project.license.as_deref() {
        ui.info("⚖", "License:", license)?;
    }

    ui.divider()?;
    ui.info("📁", "Source files:", &format_number(report.source.files))?;
    ui.info("📏", "Source lines:", &format_number(report.source.lines))?;

    ui.divider()?;

    if let Some(git) = report.git.as_ref() {
        let status = if git.changed_files == 0 {
            "clean".to_owned()
        } else {
            format!("{} changed file(s)", git.changed_files)
        };

        ui.info("🌿", "Git branch:", &git.branch)?;
        ui.info("✏️", "Working tree:", &status)?;

        if let Some(commits) = git.commits {
            ui.info("📜", "Commits:", &format_number(commits))?;
        }

        if let Some(tag) = git.latest_tag.as_deref() {
            ui.info("🏷", "Latest tag:", tag)?;
        }
    } else {
        ui.info("🌿", "Git:", "not a repository")?;
    }

    ui.divider()?;
    print_check(ui, "README", report.files.readme)?;
    print_check(ui, "LICENSE", report.files.license)?;
    print_check(ui, "CHANGELOG", report.files.changelog)?;
    print_check(ui, ".gitignore", report.files.gitignore)?;
    print_check(ui, "GitHub Actions CI", report.files.ci)?;

    ui.divider()?;
    ui.info(
        "⚡",
        "yoo:",
        &format!(
            "v{} · try `yoo project --json` for automation",
            report.yoo_version
        ),
    )?;

    Ok(())
}

fn print_check(ui: &Ui, label: &str, present: bool) -> io::Result<()> {
    let (icon, status) = if present {
        ("✓", "found")
    } else {
        ("○", "not found")
    };

    ui.info(icon, label, status)
}

fn package_manager(directory: &Path, language: &str) -> Option<String> {
    match language {
        "Rust" => Some("Cargo".to_owned()),
        "Node.js" => {
            if directory.join("pnpm-lock.yaml").is_file() {
                Some("pnpm".to_owned())
            } else if directory.join("yarn.lock").is_file() {
                Some("Yarn".to_owned())
            } else if directory.join("bun.lockb").is_file() || directory.join("bun.lock").is_file()
            {
                Some("Bun".to_owned())
            } else {
                Some("npm".to_owned())
            }
        }
        "Python" => {
            if directory.join("uv.lock").is_file() {
                Some("uv".to_owned())
            } else if directory.join("poetry.lock").is_file() {
                Some("Poetry".to_owned())
            } else if directory.join("Pipfile").is_file() {
                Some("Pipenv".to_owned())
            } else {
                Some("pip".to_owned())
            }
        }
        "Go" => Some("Go modules".to_owned()),
        "Java" => {
            if directory.join("pom.xml").is_file() {
                Some("Maven".to_owned())
            } else {
                Some("Gradle".to_owned())
            }
        }
        ".NET" => Some(".NET SDK".to_owned()),
        _ => None,
    }
}

fn count_source(directory: &Path, language: &str) -> SourceSummary {
    let mut summary = SourceSummary { files: 0, lines: 0 };
    visit_source_files(directory, source_extensions(language), &mut summary);
    summary
}

fn visit_source_files(directory: &Path, extensions: &[&str], summary: &mut SourceSummary) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_dir() {
            if should_skip_directory(&path) {
                continue;
            }

            visit_source_files(&path, extensions, summary);
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str());

        if !extension.is_some_and(|value| extensions.contains(&value)) {
            continue;
        }

        summary.files += 1;
        summary.lines += fs::read_to_string(path)
            .map(|contents| contents.lines().count())
            .unwrap_or(0);
    }
}

fn should_skip_directory(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    matches!(
        name,
        ".git" | "target" | "node_modules" | "dist" | "build" | ".next" | ".venv" | "vendor"
    )
}

fn source_extensions(language: &str) -> &'static [&'static str] {
    match language {
        "Rust" => &["rs"],
        "Node.js" => &["js", "cjs", "mjs", "jsx", "ts", "tsx"],
        "Python" => &["py"],
        "Go" => &["go"],
        "Java" => &["java", "kt", "kts"],
        ".NET" => &["cs", "fs", "vb"],
        _ => &[
            "rs", "py", "go", "java", "js", "ts", "tsx", "cs", "c", "cpp", "h", "hpp",
        ],
    }
}

fn any_file_exists(directory: &Path, candidates: &[&str]) -> bool {
    candidates.iter().any(|name| directory.join(name).is_file())
}

fn has_ci_workflow(directory: &Path) -> bool {
    let workflows = directory.join(".github").join("workflows");

    fs::read_dir(workflows)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .any(|entry| entry.path().is_file())
        })
        .unwrap_or(false)
}

fn find_license_file(directory: &Path) -> Option<String> {
    ["LICENSE", "LICENSE.md", "LICENCE", "LICENCE.md"]
        .iter()
        .find(|name| directory.join(name).is_file())
        .map(|name| (*name).to_owned())
}

fn format_number(value: usize) -> String {
    let text = value.to_string();
    let mut formatted = String::with_capacity(text.len() + text.len() / 3);

    for (index, character) in text.chars().enumerate() {
        if index > 0 && (text.len() - index) % 3 == 0 {
            formatted.push(',');
        }

        formatted.push(character);
    }

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temporary_directory() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();

        let directory =
            std::env::temp_dir().join(format!("yoo-project-test-{}-{nonce}", process::id()));

        fs::create_dir_all(directory.join("src")).expect("test directory should be created");

        directory
    }

    #[test]
    fn collects_a_rust_project_overview() {
        let directory = temporary_directory();

        fs::write(
            directory.join("Cargo.toml"),
            r#"[package]
name = "demo"
version = "1.2.3"
edition = "2024"
license = "MIT"
"#,
        )
        .expect("manifest should be written");

        fs::write(directory.join("src").join("main.rs"), "fn main() {}\n")
            .expect("source file should be written");

        fs::write(directory.join("README.md"), "# Demo\n").expect("README should be written");
        fs::write(directory.join("LICENSE"), "MIT\n").expect("license should be written");

        let report = collect(&directory);

        assert_eq!(report.project.name, "demo");
        assert_eq!(report.project.language, "Rust");
        assert_eq!(report.project.package_manager.as_deref(), Some("Cargo"));
        assert_eq!(report.project.edition.as_deref(), Some("2024"));
        assert_eq!(report.project.license.as_deref(), Some("MIT"));
        assert_eq!(report.source.files, 1);
        assert!(report.files.readme);
        assert!(report.files.license);

        fs::remove_dir_all(directory).expect("test directory should be removed");
    }

    #[test]
    fn chooses_pnpm_when_its_lockfile_exists() {
        let directory = temporary_directory();

        fs::write(directory.join("pnpm-lock.yaml"), "lockfileVersion: '9.0'\n")
            .expect("lockfile should be written");

        assert_eq!(
            package_manager(&directory, "Node.js").as_deref(),
            Some("pnpm")
        );

        fs::remove_dir_all(directory).expect("test directory should be removed");
    }

    #[test]
    fn formats_numbers_for_display() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1_234_567), "1,234,567");
    }
}
