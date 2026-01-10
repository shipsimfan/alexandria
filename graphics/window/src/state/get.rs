use crate::{CursorLock, DisplayMode, WindowState};
use alexandria_math::Vector2u;

impl WindowState {
    /// Get the current title of the window
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the current size of the client area of the window
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the current width of the client area of the window
    pub fn width(&self) -> u32 {
        self.size.x
    }

    /// Get the current height of the client area of the window
    pub fn height(&self) -> u32 {
        self.size.y
    }

    /// Get the current mode the windowing is displaying as
    pub fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Get how the cursor should be locked to the window
    pub fn cursor_lock(&self) -> CursorLock {
        self.cursor_lock
    }

    /// Is a close requested?
    pub fn is_close_requested(&self) -> bool {
        self.is_close_requested
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Is the window maximized?
    pub fn is_maximized(&self) -> bool {
        self.is_maximized
    }
}
