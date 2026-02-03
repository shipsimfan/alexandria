use linux::{
    sys::utsname::{uname, utsname},
    try_linux,
};
use std::borrow::Cow;

/// Get the current version of the running operating system
pub fn os_version() -> Result<String, linux::Error> {
    let mut info = utsname::default();
    try_linux!(uname(&mut info))?;

    let mut version_length = 0;
    for byte in &info.release {
        if *byte == 0 {
            break;
        }

        version_length += 1;
    }

    Ok(
        match String::from_utf8_lossy(unsafe {
            std::mem::transmute(&info.release[..version_length])
        }) {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        },
    )
}
