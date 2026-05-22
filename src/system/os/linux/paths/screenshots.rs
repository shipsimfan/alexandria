use crate::system::pictures_dir;

/// Get the path to the user's screenshots directory
pub fn screenshots_dir() -> Option<std::path::PathBuf> {
    pictures_dir().map(|pictures| pictures.join("Screenshots"))
}
