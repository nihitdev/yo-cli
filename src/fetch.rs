use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{git, manifest, ui::Ui};

#[derive(Debug, Clone, Serialize)]
pub struct FetchReport {
    pub yoo_version: String,
    pub environment: EnvironmentInfo,
    pub project: ProjectInfo,
    pub git: Option<GitSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnvironmentInfo {
    pub os: String,
    pub architecture: String,
    pub shell: String,
    pub terminal: String,
    pub editor: String,
    pub rustc: Option<String>,
    pub cargo: Option<String>,
    pub git: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub kind: String,
    pub manifest: Option<String>,
    pub version: Option<String>,
    pub directory: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitSummary {
    pub branch: String,
    pub changed_files: usize,
}

pub fn collect(directory: &Path) -> FetchReport {
    FetchReport {
        yoo_version: env!("CARGO_PKG_VERSION").to_owned(),
        environment: EnvironmentInfo {
            os: display_os(),
            architecture: env::consts::ARCH.to_owned(),
            shell: detect_shell(),
            terminal: detect_terminal(),
            editor: detect_editor(),
            rustc: command_version("rustc", &["--version"]),
            cargo: command_version("cargo", &["--version"]),
            git: command_version("git", &["--version"]),
        },
        project: detect_project(directory),
        git: git::inspect(directory).map(|info| GitSummary {
            branch: info.branch,
            changed_files: info.changed_files,
        }),
    }
}

pub fn to_json(report: &FetchReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

pub fn print(report: &FetchReport, ui: &Ui) -> io::Result<()> {
    ui.heading("yoo fetch — developer environment")?;
    ui.blank_line()?;

    ui.divider()?;
    ui.info(
        "🖥",
        "OS:",
        &format!(
            "{} ({})",
            report.environment.os, report.environment.architecture
        ),
    )?;
    ui.info("🐚", "Shell:", &report.environment.shell)?;
    ui.info("⌨", "Terminal:", &report.environment.terminal)?;
    ui.info("📝", "Editor:", &report.environment.editor)?;

    ui.divider()?;
    print_tool(ui, "🦀", "Rust:", report.environment.rustc.as_deref())?;
    print_tool(ui, "📦", "Cargo:", report.environment.cargo.as_deref())?;
    print_tool(ui, "🌿", "Git:", report.environment.git.as_deref())?;

    ui.divider()?;
    ui.info("📁", "Project:", &report.project.name)?;
    ui.info("🔧", "Project type:", &report.project.kind)?;

    if let Some(manifest) = report.project.manifest.as_deref() {
        ui.info("📄", "Manifest:", manifest)?;
    }

    if let Some(version) = report.project.version.as_deref() {
        ui.info("🏷", "Project version:", version)?;
    }

    if let Some(git) = report.git.as_ref() {
        let status = if git.changed_files == 0 {
            "clean".to_owned()
        } else {
            format!("{} changed file(s)", git.changed_files)
        };
        ui.info("🌿", "Git branch:", &git.branch)?;
        ui.info("✏️", "Working tree:", &status)?;
    } else {
        ui.info("🌿", "Git:", "not a repository")?;
    }

    ui.divider()?;
    ui.info(
        "⚡",
        "yoo:",
        &format!(
            "v{} · try `yoo project` for a deeper project overview",
            report.yoo_version
        ),
    )?;
    Ok(())
}

fn print_tool(ui: &Ui, icon: &str, label: &str, version: Option<&str>) -> io::Result<()> {
    ui.info(icon, label, version.unwrap_or("not found in PATH"))
}

fn command_version(program: &str, arguments: &[&str]) -> Option<String> {
    git::run_command(program, arguments)
}

fn display_os() -> String {
    match env::consts::OS {
        "windows" => "Windows".to_owned(),
        "macos" => "macOS".to_owned(),
        "linux" => "Linux".to_owned(),
        other => other.to_owned(),
    }
}

fn detect_shell() -> String {
    if cfg!(target_os = "windows") {
        if env::var_os("PSModulePath").is_some() {
            return "PowerShell".to_owned();
        }

        if let Some(shell) = env::var_os("ComSpec") {
            return executable_name(Path::new(&shell))
                .unwrap_or_else(|| "Command Prompt".to_owned());
        }
    }

    env::var_os("SHELL")
        .as_deref()
        .and_then(|shell| executable_name(Path::new(shell)))
        .unwrap_or_else(|| "not detected".to_owned())
}

fn detect_terminal() -> String {
    if env::var_os("WT_SESSION").is_some() {
        return "Windows Terminal".to_owned();
    }

    if let Some(program) = env::var_os("TERM_PROGRAM") {
        let value = program.to_string_lossy().trim().to_owned();
        if !value.is_empty() {
            return value;
        }
    }

    if env::var_os("VSCODE_GIT_IPC_HANDLE").is_some()
        || env::var_os("VSCODE_IPC_HOOK_CLI").is_some()
    {
        return "VS Code integrated terminal".to_owned();
    }

    env::var_os("TERM")
        .map(|term| term.to_string_lossy().trim().to_owned())
        .filter(|term| !term.is_empty() && term != "dumb")
        .unwrap_or_else(|| "not detected".to_owned())
}

fn detect_editor() -> String {
    if env::var_os("VSCODE_GIT_IPC_HANDLE").is_some()
        || env::var_os("VSCODE_IPC_HOOK_CLI").is_some()
    {
        return "Visual Studio Code".to_owned();
    }

    env::var_os("VISUAL")
        .or_else(|| env::var_os("EDITOR"))
        .as_deref()
        .and_then(|editor| executable_name(Path::new(editor)))
        .unwrap_or_else(|| "not detected".to_owned())
}

fn executable_name(path: &Path) -> Option<String> {
    path.file_stem()
        .or_else(|| path.file_name())
        .and_then(|name| name.to_str())
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
}

pub fn detect_project(directory: &Path) -> ProjectInfo {
    let fallback_name = directory_name(directory);

    if let Some(contents) = read_file(directory.join("Cargo.toml")) {
        let metadata = manifest::cargo(&contents);
        return ProjectInfo {
            name: metadata.name.unwrap_or(fallback_name),
            kind: "Rust".to_owned(),
            manifest: Some("Cargo.toml".to_owned()),
            version: metadata.version,
            directory: directory.display().to_string(),
        };
    }

    if let Some(contents) = read_file(directory.join("package.json")) {
        let metadata = manifest::package_json(&contents);
        return ProjectInfo {
            name: metadata.name.unwrap_or(fallback_name),
            kind: "Node.js".to_owned(),
            manifest: Some("package.json".to_owned()),
            version: metadata.version,
            directory: directory.display().to_string(),
        };
    }

    if let Some(contents) = read_file(directory.join("pyproject.toml")) {
        let metadata = manifest::pyproject(&contents);
        return project_from_metadata(
            directory,
            fallback_name,
            "Python",
            "pyproject.toml",
            metadata,
        );
    }

    if let Some(contents) = read_file(directory.join("go.mod")) {
        let metadata = manifest::go_mod(&contents);
        return project_from_metadata(directory, fallback_name, "Go", "go.mod", metadata);
    }

    if directory.join("pom.xml").is_file()
        || directory.join("build.gradle").is_file()
        || directory.join("build.gradle.kts").is_file()
    {
        let manifest = if directory.join("pom.xml").is_file() {
            "pom.xml"
        } else if directory.join("build.gradle.kts").is_file() {
            "build.gradle.kts"
        } else {
            "build.gradle"
        };
        let metadata = read_file(directory.join(manifest))
            .filter(|_| manifest == "pom.xml")
            .map(|contents| manifest::maven(&contents))
            .unwrap_or_default();
        return project_from_metadata(directory, fallback_name, "Java", manifest, metadata);
    }

    if let Some(project_file) = first_project_file(directory, |path| {
        matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("sln" | "csproj")
        )
    }) {
        let manifest_name = project_file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(".NET project")
            .to_owned();
        let metadata = read_file(project_file)
            .map(|contents| manifest::dotnet(&contents))
            .unwrap_or_default();
        return project_from_metadata(directory, fallback_name, ".NET", &manifest_name, metadata);
    }

    ProjectInfo {
        name: fallback_name,
        kind: "Generic directory".to_owned(),
        manifest: None,
        version: None,
        directory: directory.display().to_string(),
    }
}

fn project_from_metadata(
    directory: &Path,
    fallback_name: String,
    kind: &str,
    manifest: &str,
    metadata: manifest::Metadata,
) -> ProjectInfo {
    ProjectInfo {
        name: metadata.name.unwrap_or(fallback_name),
        kind: kind.to_owned(),
        manifest: Some(manifest.to_owned()),
        version: metadata.version,
        directory: directory.display().to_string(),
    }
}

fn first_project_file(directory: &Path, predicate: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let entries = fs::read_dir(directory).ok()?;

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| predicate(path))
}

fn directory_name(directory: &Path) -> String {
    directory
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("current directory")
        .to_owned()
}

fn read_file(path: PathBuf) -> Option<String> {
    fs::read_to_string(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_never_panics_for_a_temp_directory() {
        let report = collect(&std::env::temp_dir());
        assert!(!report.environment.os.is_empty());
        assert!(!report.project.name.is_empty());
    }
}
