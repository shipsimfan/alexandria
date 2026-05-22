use crate::system::home_dir;

/// Get the path to the user's videos directory
pub fn videos_dir() -> Option<std::path::PathBuf> {
    if let Ok(path) = std::env::var("XDG_VIDEOS_DIR") {
        return Some(std::path::PathBuf::from(path));
    }

    home_dir().map(|home| home.join("Videos"))
}
