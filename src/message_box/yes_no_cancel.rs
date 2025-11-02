use crate::{message_box::MessageBoxStyle, Error, Result, Window};
use std::ptr::null_mut;
use win32::{try_get_last_error, MessageBox, IDNO, IDYES, MB_YESNOCANCEL};

/// The result of a "yes", "no", and "cancel" message box
pub enum MessageBoxYncResult {
    /// The user selected "yes"
    Yes,

    /// The user selected "no"
    No,

    /// The user selected "cancel"
    Cancel,
}

/// Display a message box with "yes", "no", and "cancel" options
pub fn message_box_yes_no_cancel<LogCallbacks: crate::LogCallbacks>(
    title: &str,
    content: &str,
    style: MessageBoxStyle,
    window: Option<&mut Window<LogCallbacks>>,
) -> Result<MessageBoxYncResult> {
    let wnd = window.map(Window::handle).unwrap_or(null_mut());
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);
    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);
    let style = style.to_win32() | MB_YESNOCANCEL;

    let result = try_get_last_error!(MessageBox(wnd, content.as_ptr(), title.as_ptr(), style))
        .map_err(|error| Error::new_os("unable to display message box", error))?;

    Ok(match result {
        IDYES => MessageBoxYncResult::Yes,
        IDNO => MessageBoxYncResult::No,
        _ => MessageBoxYncResult::Cancel,
    })
}
