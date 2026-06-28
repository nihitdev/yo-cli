use std::{error::Error, fmt, fs, io, path::Path};

use serde::Deserialize;

use crate::{config, content};

const GENERAL_TIPS: &[&str] = &[
    "Commit small, meaningful changes before starting the next feature.",
    "A tiny reproducible bug report beats a vague error description.",
    "Write the test that would have caught your last bug.",
    "When stuck, reduce the problem until it looks almost silly.",
    "Make it work, make it clear, then make it fast.",
    "A clean README is part of the project, not an afterthought.",
];

const RUST_TIPS: &[&str] = &[
    "Run cargo fmt and cargo clippy before you push.",
    "Prefer Result over panic for errors a caller can recover from.",
    "Use cargo test often; small tests are cheaper than late debugging.",
    "Make ownership obvious before reaching for clone().",
];

const GIT_TIPS: &[&str] = &[
    "Use git diff before git commit when a change feels suspicious.",
    "Write commit messages that explain why, not only what.",
    "Push working milestones so your machine is never the only backup.",
    "Keep unrelated changes out of the same commit.",
];

const LINUX_TIPS: &[&str] = &[
    "Use rg before grep when you want fast codebase search.",
    "Read the man page before memorising a command from a random snippet.",
    "Check permissions before assuming a command is broken.",
    "Pipe carefully: inspect the output before adding destructive commands.",
];

pub const SAMPLE_PACK: &str = r#"# Community tip pack for yoo.
# Put more *.yaml or *.yml packs in this folder, then run `yoo tips`.

name: community
description: Tips maintained by the yoo community.
tips:
  - Share a small reproducible example when asking for help.
  - Document the weird workaround before future-you has to rediscover it.
  - A release is not done until someone can install and use it.
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TipPack {
    pub name: String,
    pub description: String,
    pub tips: Vec<String>,
    pub source: PackSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackSource {
    BuiltIn,
    Local,
}

#[derive(Debug, Deserialize)]
struct TipPackDocument {
    name: Option<String>,
    description: Option<String>,
    tips: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TipError {
    message: String,
}

impl TipError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TipError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TipError {}

pub fn list_packs() -> Result<Vec<TipPack>, TipError> {
    let mut packs = built_in_packs();

    for custom_pack in load_local_packs()? {
        if let Some(index) = packs
            .iter()
            .position(|pack| pack.name.eq_ignore_ascii_case(&custom_pack.name))
        {
            packs[index] = custom_pack;
        } else {
            packs.push(custom_pack);
        }
    }

    packs.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(packs)
}

pub fn random_tip(pack_name: &str) -> Result<String, TipError> {
    let packs = list_packs()?;
    let pack = packs
        .iter()
        .find(|pack| pack.name.eq_ignore_ascii_case(pack_name.trim()))
        .ok_or_else(|| {
            TipError::new(format!(
                "tip pack `{}` was not found; run `yoo tips` to see available packs",
                pack_name.trim()
            ))
        })?;

    if pack.tips.is_empty() {
        return Err(TipError::new(format!(
            "tip pack `{}` has no tips",
            pack.name
        )));
    }

    let index = content::random_index(pack.tips.len(), 0xD4);
    Ok(pack.tips[index].clone())
}

pub fn write_sample_pack() -> io::Result<config::WriteResult> {
    let path = config::tip_packs_dir().join("community.yaml");

    if path.exists() {
        return Ok(config::WriteResult::AlreadyExists);
    }

    fs::create_dir_all(config::tip_packs_dir())?;
    fs::write(path, SAMPLE_PACK)?;
    Ok(config::WriteResult::Created)
}

fn built_in_packs() -> Vec<TipPack> {
    vec![
        built_in(
            "general",
            "General software-building reminders.",
            GENERAL_TIPS,
        ),
        built_in(
            "rust",
            "Rust workflow and code-quality reminders.",
            RUST_TIPS,
        ),
        built_in("git", "Git workflow and backup reminders.", GIT_TIPS),
        built_in(
            "linux",
            "Terminal and Linux workflow reminders.",
            LINUX_TIPS,
        ),
    ]
}

fn built_in(name: &str, description: &str, tips: &[&str]) -> TipPack {
    TipPack {
        name: name.to_owned(),
        description: description.to_owned(),
        tips: tips.iter().map(|tip| (*tip).to_owned()).collect(),
        source: PackSource::BuiltIn,
    }
}

fn load_local_packs() -> Result<Vec<TipPack>, TipError> {
    let directory = config::tip_packs_dir();

    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(TipError::new(format!(
                "could not read tip-pack directory {}: {error}",
                directory.display()
            )));
        }
    };

    let mut packs = Vec::new();
    for entry in entries {
        let entry = entry
            .map_err(|error| TipError::new(format!("could not read tip-pack entry: {error}")))?;
        let path = entry.path();

        if !is_yaml_file(&path) {
            continue;
        }

        packs.push(read_pack_file(&path)?);
    }

    Ok(packs)
}

fn is_yaml_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("yaml" | "yml")
    )
}

fn read_pack_file(path: &Path) -> Result<TipPack, TipError> {
    let contents = fs::read_to_string(path)
        .map_err(|error| TipError::new(format!("could not read {}: {error}", path.display())))?;
    let document = serde_yaml::from_str::<TipPackDocument>(&contents)
        .map_err(|error| TipError::new(format!("invalid tip pack {}: {error}", path.display())))?;

    let fallback_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("community");
    let name = document
        .name
        .unwrap_or_else(|| fallback_name.to_owned())
        .trim()
        .to_owned();
    let description = document
        .description
        .unwrap_or_else(|| "Local community tip pack.".to_owned())
        .trim()
        .to_owned();
    let tips: Vec<String> = document
        .tips
        .into_iter()
        .map(|tip| tip.trim().to_owned())
        .filter(|tip| !tip.is_empty())
        .collect();

    if name.is_empty() {
        return Err(TipError::new(format!(
            "tip pack {} has an empty name",
            path.display()
        )));
    }

    if tips.is_empty() {
        return Err(TipError::new(format!(
            "tip pack {} has no tips",
            path.display()
        )));
    }

    Ok(TipPack {
        name,
        description,
        tips,
        source: PackSource::Local,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_general_pack_exists() {
        let packs = built_in_packs();
        assert!(packs.iter().any(|pack| pack.name == "general"));
    }

    #[test]
    fn parses_a_tip_pack_document() {
        let document: TipPackDocument = serde_yaml::from_str(
            "name: web\ndescription: Browser tips\ntips:\n  - Test the happy path\n",
        )
        .expect("tip pack should parse");
        assert_eq!(document.name.as_deref(), Some("web"));
        assert_eq!(document.tips.len(), 1);
    }
}
