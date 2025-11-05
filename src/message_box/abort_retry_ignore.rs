use crate::{message_box::MessageBoxStyle, Error, Result, Window};
use std::ptr::null_mut;
use win32::{try_get_last_error, MessageBox, IDIGNORE, IDRETRY, MB_ABORTRETRYIGNORE};

/// The result of an "abort", "retry", and "ignore" message box
pub enum MessageBoxAbiResult {
    /// The user selected "abort"
    Abort,

    /// The user selected "retry"
    Retry,

    /// The user selected "ignore"
    Ignore,
}

/// Display a message box with "abort", "retry", and "ignore" options
pub fn message_box_abort_retry_ignore<
    LogCallbacks: crate::LogCallbacks,
    Input: crate::input::Input,
>(
    title: &str,
    content: &str,
    style: MessageBoxStyle,
    window: Option<&mut Window<LogCallbacks, Input>>,
) -> Result<MessageBoxAbiResult> {
    let wnd = window.map(Window::handle).unwrap_or(null_mut());
    let mut title: Vec<u16> = title.encode_utf16().collect();
    title.push(0);
    let mut content: Vec<u16> = content.encode_utf16().collect();
    content.push(0);
    let style = style.to_win32() | MB_ABORTRETRYIGNORE;

    let result = try_get_last_error!(MessageBox(wnd, content.as_ptr(), title.as_ptr(), style))
        .map_err(|error| Error::new_os("unable to display message box", error))?;

    Ok(match result {
        IDRETRY => MessageBoxAbiResult::Retry,
        IDIGNORE => MessageBoxAbiResult::Ignore,
        _ => MessageBoxAbiResult::Abort,
    })
}
