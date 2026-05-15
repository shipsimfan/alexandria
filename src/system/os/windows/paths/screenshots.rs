use crate::system::{os::windows::paths::get_known_folder_path, pictures_dir};
use win32::knownfolders::FOLDERID_Screenshots;

/// Get the path to the user's screenshots directory
pub fn screenshots_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Screenshots) {
        return Some(path);
    }

    pictures_dir().map(|pictures| pictures.join("Screenshots"))
}
