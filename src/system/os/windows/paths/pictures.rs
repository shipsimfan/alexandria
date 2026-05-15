use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Pictures;

/// Get the path to the user's pictures directory
pub fn pictures_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Pictures) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Pictures"))
}
