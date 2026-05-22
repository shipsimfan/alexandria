use linux::{pwd::getpwuid, unistd::getuid};
use std::{ffi::CStr, path::PathBuf};

/// Get the path to the user's home directory
pub fn home_dir() -> Option<PathBuf> {
    // Try the `$HOME` environment variable first
    if let Ok(path) = std::env::var("HOME") {
        return Some(PathBuf::from(path));
    }

    // Fallback to `getpwuid`
    let passwd = unsafe { getpwuid(getuid()).as_ref() }?;
    if passwd.dir.is_null() {
        return None;
    }
    let home = unsafe { CStr::from_ptr(passwd.dir) };

    home.to_str().ok().map(PathBuf::from)
}
