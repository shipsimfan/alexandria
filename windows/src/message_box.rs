use std::{fmt::Display, ptr::null_mut};
use win32::{
    try_get_last_error, MessageBoxEx, LANG_NEUTRAL, MAKELANGID, MB_ICONERROR, MB_OK,
    MB_SETFOREGROUND, MB_TASKMODAL, SUBLANG_DEFAULT,
};

/// Displays an error message box
pub fn message_box(title: &str, text: &dyn Display) {
    let mut title: Vec<_> = title.encode_utf16().collect();
    let mut text: Vec<_> = text.to_string().encode_utf16().collect();

    title.push(0);
    text.push(0);

    try_get_last_error!(MessageBoxEx(
        null_mut(),
        text.as_ptr(),
        title.as_ptr(),
        MB_OK | MB_ICONERROR | MB_TASKMODAL | MB_SETFOREGROUND,
        MAKELANGID!(LANG_NEUTRAL, SUBLANG_DEFAULT) as _,
    ))
    .unwrap();
}
