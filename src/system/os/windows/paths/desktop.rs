use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Desktop;

/// Get the path to the user's desktop directory
pub fn desktop_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Desktop) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Desktop"))
}
