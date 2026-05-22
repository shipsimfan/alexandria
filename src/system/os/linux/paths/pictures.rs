use crate::system::home_dir;

/// Get the path to the user's pictures directory
pub fn pictures_dir() -> Option<std::path::PathBuf> {
    if let Ok(path) = std::env::var("XDG_PICTURES_DIR") {
        return Some(std::path::PathBuf::from(path));
    }

    home_dir().map(|home| home.join("Pictures"))
}
