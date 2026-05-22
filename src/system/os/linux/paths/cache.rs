use crate::system::home_dir;
use std::path::PathBuf;

/// Get the path to a directory for storing persistent cache files for the current user
#[allow(unused_variables)]
pub fn cache_dir(company: &str, game: &str) -> Option<PathBuf> {
    // Try using `$XDG_CACHE_HOME` first
    let base_path = match std::env::var("XDG_CACHE_HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            // Fallback to `$HOME/.cache`
            let home = home_dir()?;
            home.join(".cache")
        }
    };

    // Convert the game name to a valid directory name (e.g. "My Game" -> "my-game")
    let game = game.to_lowercase().replace(' ', "-");

    Some(base_path.join(game))
}
