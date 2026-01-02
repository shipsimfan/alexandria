use crate::{DisplayMode, WindowState};
use alexandria_math::{Vector2i, Vector2u};

impl WindowState {
    /// Set the current title of the window
    pub(crate) fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Set the current position of the top-left of the client area of the window
    pub(crate) const fn set_position(&mut self, position: Vector2i) {
        self.position = position;
    }

    /// Set the current size of the client area of the window
    pub(crate) const fn set_size(&mut self, size: Vector2u) {
        self.size = size;
    }

    /// Set the current mode the windowing is displaying as
    pub(crate) const fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.display_mode = display_mode;
    }

    /// Set if the window is currently running
    pub(crate) const fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    /// Set if the window is currently focused
    pub(crate) const fn set_is_focused(&mut self, is_focused: bool) {
        self.is_focused = is_focused;
    }

    /// Set if the window is being actively moved or resized
    pub(crate) const fn set_is_changing(&mut self, is_changing: bool) {
        self.is_changing = is_changing;
    }
}
