use std::{
    env,
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(windows)]
const EDITOR_CANDIDATES: &[&str] = &["code", "cursor", "zed", "notepad"];
#[cfg(not(windows))]
const EDITOR_CANDIDATES: &[&str] = &["code", "cursor", "zed", "nvim", "vim", "vi"];

pub fn open(
    override_editor: Option<&str>,
    configured_editor: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let editor = select_editor(override_editor, configured_editor)?;
    let directory = env::current_dir()?;

    println!(
        "Opening {} in {}...",
        directory.display(),
        editor.to_string_lossy()
    );

    let status = Command::new(&editor)
        .arg(&directory)
        .status()
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "could not launch editor `{}`: {error}",
                    editor.to_string_lossy()
                ),
            )
        })?;

    if !status.success() {
        return Err(io::Error::other(format!(
            "editor `{}` exited with status {status}",
            editor.to_string_lossy()
        ))
        .into());
    }

    Ok(())
}

fn select_editor(
    override_editor: Option<&str>,
    configured_editor: Option<&str>,
) -> Result<OsString, io::Error> {
    if let Some(editor) = override_editor {
        return Ok(resolve_editor(OsString::from(editor)));
    }

    if let Some(editor) = configured_editor {
        return Ok(resolve_editor(OsString::from(editor)));
    }

    for variable in ["VISUAL", "EDITOR"] {
        if let Some(editor) = nonempty_env(variable) {
            return Ok(resolve_editor(editor));
        }
    }

    EDITOR_CANDIDATES
        .iter()
        .find_map(|editor| executable_on_path(editor))
        .map(PathBuf::into_os_string)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "no editor found; set $VISUAL or $EDITOR, or use `yoo edit --editor <EDITOR>`",
            )
        })
}

fn resolve_editor(editor: OsString) -> OsString {
    editor
        .to_str()
        .and_then(executable_on_path)
        .map(PathBuf::into_os_string)
        .unwrap_or(editor)
}

fn nonempty_env(name: &str) -> Option<OsString> {
    env::var_os(name).filter(|value| !value.is_empty())
}

fn executable_on_path(program: &str) -> Option<PathBuf> {
    let path = Path::new(program);
    if path.components().count() > 1 {
        return executable_file(path).then(|| path.to_path_buf());
    }

    let paths = env::var_os("PATH")?;

    env::split_paths(&paths).find_map(|directory| {
        executable_candidates(&directory, program).find(|path| executable_file(path))
    })
}

fn executable_candidates(directory: &Path, program: &str) -> impl Iterator<Item = PathBuf> {
    let base = directory.join(program);
    let mut candidates = Vec::new();

    #[cfg(windows)]
    if base.extension().is_none() {
        let extensions =
            env::var_os("PATHEXT").unwrap_or_else(|| OsString::from(".COM;.EXE;.BAT;.CMD"));
        candidates.extend(
            extensions
                .to_string_lossy()
                .split(';')
                .filter(|extension| !extension.is_empty())
                .map(|extension| directory.join(format!("{program}{extension}"))),
        );
    }

    candidates.push(base);

    candidates.into_iter()
}

fn executable_file(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };

    if !metadata.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }

    #[cfg(not(unix))]
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_editor_has_priority() {
        assert_eq!(
            select_editor(Some("my-editor"), Some("configured-editor"))
                .expect("explicit editor should be selected"),
            OsString::from("my-editor")
        );
    }

    #[test]
    fn configured_editor_is_used_without_an_override() {
        assert_eq!(
            select_editor(None, Some("configured-editor"))
                .expect("configured editor should be selected"),
            OsString::from("configured-editor")
        );
    }

    #[test]
    fn candidate_paths_include_the_plain_program_name() {
        let directory = Path::new("editors");
        let candidates: Vec<_> = executable_candidates(directory, "code").collect();
        assert!(candidates.contains(&directory.join("code")));

        #[cfg(windows)]
        assert_ne!(candidates.first(), Some(&directory.join("code")));
    }
}
