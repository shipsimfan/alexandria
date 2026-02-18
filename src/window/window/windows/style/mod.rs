use win32::DWORD;

mod client_to_window;
mod default;

/// The style that can be applied to a window
pub(in crate::window) struct WindowStyle {
    /// The style value
    pub style: DWORD,

    /// The extended style value
    pub ex_style: DWORD,
}
