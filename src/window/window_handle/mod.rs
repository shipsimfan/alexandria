use win32::HWND;

mod create;
mod deref;
mod drop;
mod get_size_and_position;
mod set_display_mode_size_and_position;
mod set_title;

/// A handle to a raw Win32 window
pub(in crate::window) struct WindowHandle {
    /// The handle itself
    handle: HWND,
}
