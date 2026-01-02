use alexandria_math::{Vector2i, Vector2u};

mod display_mode;

mod get;
mod new;
mod set;

pub use display_mode::DisplayMode;

/// The current state of a window
#[derive(Debug, Clone)]
pub struct WindowState {
    /// The current title of the window
    title: String,

    /// The current position of the top-left of the client area of the window
    position: Vector2i,

    /// The current size of the client area of the window
    size: Vector2u,

    /// The current mode the windowing is displaying as
    display_mode: DisplayMode,

    /// Is the window running?
    is_running: bool,

    /// Is this window the one being focused on?
    is_focused: bool,

    /// Is the window being actively moved or resized?
    is_changing: bool,

    /// Is the window minimized?
    is_minimized: bool,

    /// Is the window maximized?
    is_maximized: bool,
}
