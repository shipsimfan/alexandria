use crate::{DisplayMode, WindowState};
use alexandria_math::{Vector2i, Vector2u};

impl WindowState {
    /// Create a new [`WindowState`]
    pub(crate) const fn new(
        title: String,
        position: Vector2i,
        size: Vector2u,
        display_mode: DisplayMode,
    ) -> WindowState {
        WindowState {
            title,
            position,
            size,
            display_mode,
            is_running: true,
            is_focused: false,
            is_changing: false,
            is_minimized: false,
            is_maximized: false,
        }
    }
}
