use crate::{CursorLock, DisplayMode, WindowState};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl WindowState {
    /// Create a new [`WindowState`]
    pub(crate) fn new(
        title: Cow<'static, str>,
        size: Vector2u,
        display_mode: DisplayMode,
    ) -> WindowState {
        WindowState {
            title,
            size,
            display_mode,
            cursor_lock: CursorLock::Unlocked,
            is_close_requested: false,
            is_focused: true,
            is_maximized: false,
        }
    }
}
