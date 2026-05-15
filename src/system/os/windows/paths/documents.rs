use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Documents;

/// Get the path to the user's documents directory
pub fn documents_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Documents) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Documents"))
}
