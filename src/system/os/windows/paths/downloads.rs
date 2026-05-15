use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Downloads;

/// Get the path to the user's downloads directory
pub fn downloads_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Downloads) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Downloads"))
}
