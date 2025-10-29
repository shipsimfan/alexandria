use crate::{message_box::MessageBoxStyle, Error, Result, Window};
use std::ptr::null_mut;
use win32::{try_get_last_error, MessageBox, IDOK, MB_OKCANCEL};

/// The result of a "ok" and "cancel" message box
pub enum MessageBoxOcResult {
    /// The user selected "ok"
    Ok,

    /// The user selected "cancel"
    Cancel,
}

/// Display a message box with "ok" and "cancel" options
pub fn message_box_ok_cancel<LogCallbacks: crate::LogCallbacks>(
    title: &str,
    content: &str,
    style: MessageBoxStyle,
    window: Option<&mut Window<LogCallbacks>>,
) -> Result<MessageBoxOcResult> {
    let wnd = window.map(Window::handle).unwrap_or(null_mut());
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);
    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);
    let style = style.to_win32() | MB_OKCANCEL;

    let result = try_get_last_error!(MessageBox(wnd, title.as_ptr(), content.as_ptr(), style))
        .map_err(|error| Error::new_os("unable to display message box", error))?;

    Ok(match result {
        IDOK => MessageBoxOcResult::Ok,
        _ => MessageBoxOcResult::Cancel,
    })
}
