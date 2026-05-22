use crate::system::home_dir;
use std::path::PathBuf;

/// Get the path to a directory for storing game data for the current user
#[allow(unused_variables)]
pub fn game_data_dir(company: &str, game: &str) -> Option<PathBuf> {
    // Try using `$XDG_DATA_HOME` first
    let base_path = match std::env::var("XDG_DATA_HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            // Fallback to `$HOME/.local/share`
            let home = home_dir()?;
            home.join(".local").join("share")
        }
    };

    // Convert the game name to a valid directory name (e.g. "My Game" -> "my-game")
    let game = game.to_lowercase().replace(' ', "-");

    Some(base_path.join(game))
}
