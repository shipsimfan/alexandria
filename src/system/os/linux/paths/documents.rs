use crate::system::home_dir;

/// Get the path to the user's documents directory
pub fn documents_dir() -> Option<std::path::PathBuf> {
    if let Ok(path) = std::env::var("XDG_DOCUMENTS_DIR") {
        return Some(std::path::PathBuf::from(path));
    }

    home_dir().map(|home| home.join("Documents"))
}
