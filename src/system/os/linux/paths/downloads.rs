use crate::system::home_dir;

/// Get the path to the user's downloads directory
pub fn downloads_dir() -> Option<std::path::PathBuf> {
    if let Ok(path) = std::env::var("XDG_DOWNLOAD_DIR") {
        return Some(std::path::PathBuf::from(path));
    }

    home_dir().map(|home| home.join("Downloads"))
}
