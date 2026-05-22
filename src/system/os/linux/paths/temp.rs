use std::path::PathBuf;

/// Get the path to a directory for storing temporary files for the current user
pub fn temp_dir() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("TMPDIR") {
        return Some(std::path::PathBuf::from(path));
    }

    Some(PathBuf::from("/tmp/"))
}
