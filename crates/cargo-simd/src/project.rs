use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct ProjectInfo {
    pub root: Option<PathBuf>,
    pub has_lockfile: bool,
    pub target_directory: Option<PathBuf>,
    pub target_size_bytes: Option<u64>,
}

pub fn inspect(deep: bool) -> ProjectInfo {
    let root = find_workspace_root().or_else(find_project_root);

    let Some(root) = root else {
        return ProjectInfo {
            root: None,
            has_lockfile: false,
            target_directory: None,
            target_size_bytes: None,
        };
    };

    let has_lockfile = root.join("Cargo.lock").is_file();

    let target = root.join("target");
    let target_directory = target.is_dir().then_some(target);

    let target_size_bytes = if deep {
        target_directory.as_deref().and_then(directory_size)
    } else {
        None
    };

    ProjectInfo {
        root: Some(root),
        has_lockfile,
        target_directory,
        target_size_bytes,
    }
}

fn find_workspace_root() -> Option<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let manifest = String::from_utf8(output.stdout).ok()?;
    let manifest = PathBuf::from(manifest.trim());

    manifest.parent().map(Path::to_path_buf)
}

fn find_project_root() -> Option<PathBuf> {
    let mut current = env::current_dir().ok()?;

    loop {
        if current.join("Cargo.toml").is_file() {
            return Some(current);
        }

        if !current.pop() {
            return None;
        }
    }
}

fn directory_size(path: &Path) -> Option<u64> {
    let entries = fs::read_dir(path).ok()?;
    let mut total = 0_u64;

    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };

        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            if let Some(size) = directory_size(&entry.path()) {
                total = total.saturating_add(size);
            }

            continue;
        }

        if file_type.is_file()
            && let Ok(metadata) = entry.metadata()
        {
            total = total.saturating_add(metadata.len());
        }
    }

    Some(total)
}
