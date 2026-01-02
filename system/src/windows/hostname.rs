use std::borrow::Cow;
use win32::{
    try_wsa_get_last_error,
    winsock2::{WSACleanup, WSADATA, WSAStartup, gethostname},
};

/// Get the hostname of the current computer
pub fn hostname() -> Result<String, win32::Error> {
    let mut wsa = WSADATA::default();
    try_wsa_get_last_error!(WSAStartup(0x0202, &mut wsa))?;

    let mut hostname = [0u8; 257];
    try_wsa_get_last_error!(gethostname(
        hostname.as_mut_ptr().cast(),
        hostname.len() as _
    ))?;

    try_wsa_get_last_error!(WSACleanup())?;

    let mut hostname_length = 0;
    for byte in &hostname {
        if *byte == 0 {
            break;
        }

        hostname_length += 1;
    }

    Ok(
        match String::from_utf8_lossy(&hostname[..hostname_length]) {
            Cow::Owned(owned) => owned,
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
        },
    )
}
