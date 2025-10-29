use crate::{message_box::MessageBoxStyle, Error, Result, Window};
use std::ptr::null_mut;
use win32::{try_get_last_error, MessageBox, MB_OK};

/// Display a message box with an "ok" option
pub fn message_box_ok<LogCallbacks: crate::LogCallbacks>(
    title: &str,
    content: &str,
    style: MessageBoxStyle,
    window: Option<&mut Window<LogCallbacks>>,
) -> Result<()> {
    let wnd = window.map(Window::handle).unwrap_or(null_mut());
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);
    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);
    let style = style.to_win32() | MB_OK;

    try_get_last_error!(MessageBox(wnd, title.as_ptr(), content.as_ptr(), style))
        .map_err(|error| Error::new_os("unable to display message box", error))
        .map(|_| ())
}
