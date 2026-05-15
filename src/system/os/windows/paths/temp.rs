use std::path::PathBuf;
use win32::{GetTempPath, MAX_PATH, try_get_last_error};

/// Get the path to a directory for storing temporary files for the current user
pub fn temp_dir() -> Option<PathBuf> {
    let mut buffer = [0; MAX_PATH + 1];

    try_get_last_error!(GetTempPath(buffer.len() as _, buffer.as_mut_ptr())).ok()?;

    let len = buffer.iter().position(|&c| c == 0).unwrap_or(0);
    String::from_utf16(&buffer[..len]).map(PathBuf::from).ok()
}
