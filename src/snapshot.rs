use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{fetch, git, project, ui::Ui};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolVersions {
    pub rustc: Option<String>,
    pub cargo: Option<String>,
    pub git: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snapshot {
    pub timestamp: u128,
    pub working_directory: String,
    pub project_name: String,
    pub project_type: String,
    pub project_version: Option<String>,
    pub git_branch: Option<String>,
    pub git_changed_files: Option<usize>,
    pub latest_commit_hash: Option<String>,
    pub latest_commit_message: Option<String>,
    pub source_file_count: usize,
    pub source_line_count: usize,
    pub tools: ToolVersions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotComparison {
    pub branch: Option<(String, String)>,
    pub changed_files: Option<(usize, usize)>,
    pub source_files_difference: i128,
    pub source_lines_difference: i128,
    pub changed_versions: Vec<(String, Option<String>, Option<String>)>,
}

impl Snapshot {
    pub fn collect(directory: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let project_report = project::collect(directory)?;
        let environment = fetch::collect(directory)?.environment;
        let commit = if project_report.git.is_some() {
            git::latest_commit(directory)?
        } else {
            None
        };
        Ok(Self {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
            working_directory: directory.display().to_string(),
            project_name: project_report.project.name,
            project_type: project_report.project.language,
            project_version: project_report.project.version,
            git_branch: project_report.git.as_ref().map(|git| git.branch.clone()),
            git_changed_files: project_report.git.as_ref().map(|git| git.changed_files),
            latest_commit_hash: commit.as_ref().map(|commit| commit.hash.clone()),
            latest_commit_message: commit.map(|commit| commit.message),
            source_file_count: project_report.source.files,
            source_line_count: project_report.source.lines,
            tools: ToolVersions {
                rustc: environment.rustc,
                cargo: environment.cargo,
                git: environment.git,
            },
        })
    }

    pub fn compare(old: &Self, new: &Self) -> SnapshotComparison {
        let changed_versions = [
            (
                "Project",
                old.project_version.clone(),
                new.project_version.clone(),
            ),
            ("Rustc", old.tools.rustc.clone(), new.tools.rustc.clone()),
            ("Cargo", old.tools.cargo.clone(), new.tools.cargo.clone()),
            ("Git", old.tools.git.clone(), new.tools.git.clone()),
        ]
        .into_iter()
        .filter(|(_, before, after)| before != after)
        .map(|(name, before, after)| (name.to_owned(), before, after))
        .collect();
        SnapshotComparison {
            branch: (old.git_branch != new.git_branch).then(|| {
                (
                    display_option(&old.git_branch),
                    display_option(&new.git_branch),
                )
            }),
            changed_files: (old.git_changed_files != new.git_changed_files).then(|| {
                (
                    old.git_changed_files.unwrap_or(0),
                    new.git_changed_files.unwrap_or(0),
                )
            }),
            source_files_difference: new.source_file_count as i128 - old.source_file_count as i128,
            source_lines_difference: new.source_line_count as i128 - old.source_line_count as i128,
            changed_versions,
        }
    }
}

pub fn save(directory: &Path) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let snapshot = Snapshot::collect(directory)?;
    let snapshot_dir = directory.join(".yoo").join("snapshots");
    fs::create_dir_all(&snapshot_dir)?;
    let mut path = snapshot_path(&snapshot_dir, snapshot.timestamp);
    let mut suffix = 1;
    while path.exists() {
        path = snapshot_path(&snapshot_dir, snapshot.timestamp + suffix);
        suffix += 1;
    }
    let mut temporary = tempfile::NamedTempFile::new_in(&snapshot_dir)?;
    temporary.write_all(&serde_json::to_vec_pretty(&snapshot)?)?;
    temporary.persist(&path)?;
    Ok(snapshot)
}

pub fn load_all(directory: &Path) -> Result<Vec<Snapshot>, Box<dyn std::error::Error>> {
    let snapshot_dir = directory.join(".yoo").join("snapshots");
    let mut snapshots = Vec::new();
    if !snapshot_dir.is_dir() {
        return Ok(snapshots);
    }
    for entry in fs::read_dir(snapshot_dir)? {
        let entry = entry?;
        if entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
            != Some("json")
        {
            continue;
        }
        snapshots.push(serde_json::from_slice(&fs::read(entry.path())?)?);
    }
    snapshots.sort_by_key(|snapshot| std::cmp::Reverse(snapshot.timestamp));
    Ok(snapshots)
}

pub fn print_saved(snapshot: &Snapshot, ui: &Ui) -> io::Result<()> {
    ui.info("📸", "Snapshot:", &format_timestamp(snapshot.timestamp))?;
    ui.info(
        "📁",
        "Project:",
        &format!("{} ({})", snapshot.project_name, snapshot.project_type),
    )?;
    ui.info(
        "🌿",
        "Git:",
        snapshot.git_branch.as_deref().unwrap_or("not a repository"),
    )?;
    ui.info(
        "📏",
        "Source:",
        &format!(
            "{} files · {} lines",
            snapshot.source_file_count, snapshot.source_line_count
        ),
    )
}

pub fn print_list(snapshots: &[Snapshot], ui: &Ui) -> io::Result<()> {
    ui.heading("yoo snapshot — saved snapshots")?;
    if snapshots.is_empty() {
        return ui.info("○", "Snapshots:", "none saved");
    }
    ui.blank_line()?;
    for snapshot in snapshots {
        print_saved(snapshot, ui)?;
    }
    Ok(())
}

pub fn print_compare(old: &Snapshot, new: &Snapshot, ui: &Ui) -> io::Result<()> {
    let comparison = Snapshot::compare(old, new);
    ui.heading("yoo snapshot — comparison")?;
    ui.info(
        "📸",
        "Snapshots:",
        &format!(
            "{} → {}",
            format_timestamp(old.timestamp),
            format_timestamp(new.timestamp)
        ),
    )?;
    if let Some((before, after)) = comparison.branch {
        ui.info("🌿", "Git branch:", &format!("{before} → {after}"))?;
    }
    if let Some((before, after)) = comparison.changed_files {
        ui.info("✏️", "Changed files:", &format!("{before} → {after}"))?;
    }
    ui.info(
        "📁",
        "Source files:",
        &format_difference(comparison.source_files_difference),
    )?;
    ui.info(
        "📏",
        "Source lines:",
        &format_difference(comparison.source_lines_difference),
    )?;
    for (name, before, after) in comparison.changed_versions {
        ui.info(
            "🛠",
            &format!("{name} version:"),
            &format!("{} → {}", display_option(&before), display_option(&after)),
        )?;
    }
    Ok(())
}

fn snapshot_path(directory: &Path, timestamp: u128) -> PathBuf {
    directory.join(format!("{timestamp}.json"))
}

fn display_option(value: &Option<String>) -> String {
    value.as_deref().unwrap_or("not available").to_owned()
}

fn format_timestamp(timestamp: u128) -> String {
    let seconds = (timestamp / 1_000) as i64;
    let days = seconds.div_euclid(86_400);
    let time = seconds.rem_euclid(86_400);

    // Convert days since the Unix epoch into a Gregorian calendar date.
    let shifted_days = days + 719_468;
    let era = if shifted_days >= 0 {
        shifted_days
    } else {
        shifted_days - 146_096
    } / 146_097;
    let day_of_era = shifted_days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);

    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02} UTC",
        time / 3_600,
        time % 3_600 / 60,
        time % 60
    )
}

fn format_difference(value: i128) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(timestamp: u128, files: usize, lines: usize) -> Snapshot {
        Snapshot {
            timestamp,
            working_directory: "/tmp/project".into(),
            project_name: "demo".into(),
            project_type: "Rust".into(),
            project_version: Some("0.1.0".into()),
            git_branch: Some("main".into()),
            git_changed_files: Some(1),
            latest_commit_hash: Some("abc".into()),
            latest_commit_message: Some("Initial".into()),
            source_file_count: files,
            source_line_count: lines,
            tools: ToolVersions {
                rustc: Some("rustc 1".into()),
                cargo: Some("cargo 1".into()),
                git: Some("git 1".into()),
            },
        }
    }

    #[test]
    fn snapshot_serializes_and_deserializes() {
        let snapshot = sample(1, 2, 3);
        let json = serde_json::to_string(&snapshot).unwrap();
        assert_eq!(serde_json::from_str::<Snapshot>(&json).unwrap(), snapshot);
    }

    #[test]
    fn formats_unix_epoch_timestamp_for_display() {
        assert_eq!(format_timestamp(0), "1970-01-01 00:00:00 UTC");
    }

    #[test]
    fn snapshots_are_ordered_newest_first() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".yoo/snapshots");
        fs::create_dir_all(&path).unwrap();
        for snapshot in [sample(10, 1, 1), sample(30, 1, 1), sample(20, 1, 1)] {
            fs::write(
                path.join(format!("{}.json", snapshot.timestamp)),
                serde_json::to_vec(&snapshot).unwrap(),
            )
            .unwrap();
        }
        let snapshots = load_all(dir.path()).unwrap();
        assert_eq!(
            snapshots
                .iter()
                .map(|snapshot| snapshot.timestamp)
                .collect::<Vec<_>>(),
            vec![30, 20, 10]
        );
    }

    #[test]
    fn comparison_reports_differences() {
        let old = sample(1, 2, 10);
        let mut new = sample(2, 3, 7);
        new.git_branch = Some("feature".into());
        new.tools.git = Some("git 2".into());
        let comparison = Snapshot::compare(&old, &new);
        assert_eq!(comparison.source_files_difference, 1);
        assert_eq!(comparison.source_lines_difference, -3);
        assert_eq!(comparison.branch, Some(("main".into(), "feature".into())));
        assert_eq!(comparison.changed_versions.len(), 1);
        let mut no_git = old;
        no_git.git_changed_files = None;
        assert_eq!(Snapshot::compare(&no_git, &new).changed_files, Some((0, 1)));
    }
}
