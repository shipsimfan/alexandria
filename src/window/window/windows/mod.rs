mod class;
mod get;
mod standard_wnd_proc;
mod style;
mod win32;

mod new;
mod set_id;

pub(in crate::window) use class::{WindowClass, WindowProc};
pub(in crate::window) use standard_wnd_proc::StandardWndProc;
pub(in crate::window) use style::WindowStyle;
pub(in crate::window) use win32::Win32Window;

use crate::{Id, window::display::DisplayInner};

/// The Windows-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) struct WindowInner<UserEvent: 'static + Send> {
    /// The raw window
    window: Win32Window<StandardWndProc<UserEvent>>,

    /// If this window is currently in fullscreen mode, the display it's fullscreen on
    fullscreen_display: Option<Id<DisplayInner<UserEvent>>>,
}
