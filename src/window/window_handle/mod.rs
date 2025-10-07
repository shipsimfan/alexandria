use win32::{DWORD, HWND};

mod create;
mod deref;
mod drop;
mod get_size_and_position;
mod set_size_and_position;

/// A handle to a raw Win32 window
pub(in crate::window) struct WindowHandle {
    /// The handle itself
    handle: HWND,

    /// The style the window uses
    style: DWORD,

    /// The extended style the window uses
    ex_style: DWORD,
}
