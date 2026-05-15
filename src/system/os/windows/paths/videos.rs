use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Videos;

/// Get the path to the user's videos directory
pub fn videos_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Videos) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Videos"))
}
