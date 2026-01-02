use win32::HWND;

mod clip_cursor;
mod deref;
mod drop;
mod get_size;
mod new;
mod set_display_mode;
mod set_size;
mod set_title;

/// A Win32 window
pub(in crate::platform::windows) struct WindowHandle {
    /// The handle to the window
    handle: HWND,
}
