use crate::system::{home_dir, os::windows::paths::get_known_folder_path};
use win32::knownfolders::FOLDERID_Music;

/// Get the path to the user's music directory
pub fn music_dir() -> Option<std::path::PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_Music) {
        return Some(path);
    }

    home_dir().map(|home| home.join("Music"))
}
