use std::path::{Path, PathBuf};

/// Get the path to the root of the git repo
pub(in crate::git) fn git_repo_path() -> Option<PathBuf> {
    let base_path = Path::new(".git");
    if !base_path.exists() {
        return None;
    }

    if base_path.is_dir() {
        return Some(base_path.to_path_buf());
    }

    let contents = std::fs::read_to_string(base_path).expect("cannot read `.git` file");
    let path = contents
        .strip_prefix("gitdir: ")
        .expect("invalid `.git` file")
        .trim();

    Some(PathBuf::from(path))
}
