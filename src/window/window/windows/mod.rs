use standard_wnd_proc::StandardWndProc;

mod class;
mod standard_wnd_proc;
mod style;
mod win32;

pub(in crate::window) use class::{WindowClass, WindowProc};
pub(in crate::window) use style::WindowStyle;
pub(in crate::window) use win32::Win32Window;

/// The Windows-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) struct WindowInner {
    /// The raw window
    window: Win32Window<StandardWndProc>,
}
