use crate::{message_box::MessageBoxStyle, Error, Result, Window};
use std::ptr::null_mut;
use win32::{try_get_last_error, MessageBox, IDCONTINUE, IDTRYAGAIN, MB_CANCELTRYCONTINUE};

/// The result of a "cancel", "try", and "continue" message box
pub enum MessageBoxCtcResult {
    /// The user selected "cancel"
    Cancel,

    /// The user selected "try"
    Try,

    /// The user selected "continue"
    Continue,
}

/// Display a message box with "cancel", "try", and "continue" options
pub fn message_box_cancel_try_continue<
    LogCallbacks: crate::LogCallbacks,
    Input: crate::input::Input,
>(
    title: &str,
    content: &str,
    style: MessageBoxStyle,
    window: Option<&mut Window<LogCallbacks, Input>>,
) -> Result<MessageBoxCtcResult> {
    let wnd = window.map(Window::handle).unwrap_or(null_mut());
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);
    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);
    let style = style.to_win32() | MB_CANCELTRYCONTINUE;

    let result = try_get_last_error!(MessageBox(wnd, content.as_ptr(), title.as_ptr(), style))
        .map_err(|error| Error::new_os("unable to display message box", error))?;

    Ok(match result {
        IDTRYAGAIN => MessageBoxCtcResult::Try,
        IDCONTINUE => MessageBoxCtcResult::Continue,
        _ => MessageBoxCtcResult::Cancel,
    })
}
