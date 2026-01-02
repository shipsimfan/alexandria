use crate::{CursorLock, DisplayMode, WindowState};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl WindowState {
    /// Set the current title of the window
    pub(crate) fn set_title(&mut self, title: Cow<'static, str>) {
        self.title = title;
    }

    /// Set the current size of the client area of the window
    pub(crate) fn set_size(&mut self, size: Vector2u) {
        self.size = size;
    }

    /// Set the current mode the windowing is displaying as
    pub(crate) fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.display_mode = display_mode;
    }

    /// Get how the cursor should be locked to the window
    pub(crate) fn set_cursor_lock(&mut self, cursor_lock: CursorLock) {
        self.cursor_lock = cursor_lock;
    }

    /// Set if a close has been requested
    pub(crate) fn set_is_close_requested(&mut self, is_close_requested: bool) {
        self.is_close_requested = is_close_requested;
    }

    /// Set if the window is currently focused
    pub(crate) fn set_is_focused(&mut self, is_focused: bool) {
        self.is_focused = is_focused;
    }

    /// Set if the window is being actively  resized
    pub(crate) fn set_is_resizing(&mut self, is_resizing: bool) {
        self.is_resizing = is_resizing;
    }

    /// Set if the window is maximized
    pub(crate) fn set_is_maximized(&mut self, is_maximized: bool) {
        self.is_maximized = is_maximized;
    }
}
