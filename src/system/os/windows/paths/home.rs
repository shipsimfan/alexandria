use crate::system::os::windows::paths::get_known_folder_path;
use std::path::PathBuf;
use win32::knownfolders::FOLDERID_Profile;

/// Get the path to the user's home directory
pub fn home_dir() -> Option<PathBuf> {
    // Try the known folder first
    if let Some(path) = get_known_folder_path(&FOLDERID_Profile) {
        return Some(path);
    }

    // Fallback to `%USERPROFILE%``
    if let Ok(path) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(path));
    }

    // Finally fallback to `%HOMEDRIVE%%HOMEPATH%`
    let homedrive = std::env::var("HOMEDRIVE").ok()?;
    let homepath = std::env::var("HOMEPATH").ok()?;
    Some(PathBuf::from(format!("{}{}", homedrive, homepath)))
}
