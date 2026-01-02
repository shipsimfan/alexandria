use win32::HWND;

mod deref;
mod drop;
mod get_size_and_position;
mod new;
mod set_display_mode_size_and_position;
mod set_title;

/// A Win32 window
pub(in crate::platform::windows) struct WindowHandle {
    /// The handle to the window
    handle: HWND,
}
