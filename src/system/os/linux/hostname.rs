use linux::{
    sys::utsname::{uname, utsname},
    try_linux,
};
use std::borrow::Cow;

/// Get the hostname of the current computer
pub fn hostname() -> Result<String, linux::Error> {
    let mut info = utsname::default();
    try_linux!(uname(&mut info))?;

    let mut hostname_length = 0;
    for byte in &info.nodename {
        if *byte == 0 {
            break;
        }

        hostname_length += 1;
    }

    Ok(
        match String::from_utf8_lossy(unsafe {
            std::mem::transmute(&info.nodename[..hostname_length])
        }) {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        },
    )
}
