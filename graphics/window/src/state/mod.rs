use alexandria_math::Vector2u;
use std::borrow::Cow;

mod cursor_lock;
mod display_mode;

mod get;
mod new;
mod set;

pub use cursor_lock::CursorLock;
pub use display_mode::DisplayMode;

/// The current state of a window
#[derive(Debug, Clone)]
pub struct WindowState {
    /// The current title of the window
    title: Cow<'static, str>,

    /// The current size of the client area of the window
    size: Vector2u,

    /// The current mode the windowing is displaying as
    display_mode: DisplayMode,

    /// How the cursor should be locked to the window
    cursor_lock: CursorLock,

    /// Has a close been requested?
    is_close_requested: bool,

    /// Is this window the one being focused on?
    is_focused: bool,

    /// Is the window being actively resized?
    is_resizing: bool,

    /// Is the window maximized?
    is_maximized: bool,
}
