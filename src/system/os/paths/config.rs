use crate::system::game_data_dir;
use std::path::PathBuf;

/// Get the path to a directory for storing configuration files for the current user
pub fn config_dir(company: &str, game: &str) -> Option<PathBuf> {
    let game_data_dir = game_data_dir(company, game)?;

    Some(game_data_dir.join("config"))
}
