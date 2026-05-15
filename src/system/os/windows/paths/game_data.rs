use crate::system::os::windows::paths::get_known_folder_path;
use std::path::PathBuf;
use win32::knownfolders::FOLDERID_LocalAppData;

/// Get the path to a directory for storing game data for the current user
pub fn game_data_dir(company: &str, game: &str) -> Option<PathBuf> {
    let local_app_data = get_local_app_data()?;

    Some(local_app_data.join(company).join(game))
}

/// Get the path to the local application data directory for the current user
fn get_local_app_data() -> Option<PathBuf> {
    if let Some(path) = get_known_folder_path(&FOLDERID_LocalAppData) {
        return Some(path);
    }

    std::env::var("LOCALAPPDATA").ok().map(PathBuf::from)
}
