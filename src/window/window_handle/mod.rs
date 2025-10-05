use win32::HWND;

mod create;
mod deref;
mod get_rect;
mod drop;

/// A handle to a raw Win32 window
pub(in crate::window) struct WindowHandle {
    /// The handle itself
    handle: HWND,
}
