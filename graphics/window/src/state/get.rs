use crate::{DisplayMode, WindowState};
use alexandria_math::{Vector2i, Vector2u};

impl WindowState {
    /// Get the current title of the window
    pub const fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Get the current position of the top-left of the client area of the window
    pub const fn position(&self) -> Vector2i {
        self.position
    }

    /// Get the current position of the left side of the client area of the window
    pub const fn left(&self) -> i32 {
        self.position.x
    }

    /// Get the current position of the top of the client area of the window
    pub const fn top(&self) -> i32 {
        self.position.y
    }

    /// Get the current size of the client area of the window
    pub const fn size(&self) -> Vector2u {
        self.size
    }

    /// Get the current width of the client area of the window
    pub const fn width(&self) -> u32 {
        self.size.x
    }

    /// Get the current height of the client area of the window
    pub const fn height(&self) -> u32 {
        self.size.y
    }

    /// Get the current position of the right side of the client area of the window
    pub const fn right(&self) -> i32 {
        self.position.x + self.size.x as i32
    }

    /// Get the current position of the bottom of the client area of the window
    pub const fn bottom(&self) -> i32 {
        self.position.y + self.size.y as i32
    }

    /// Get the current mode the windowing is displaying as
    pub const fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Is the window currently running?
    pub const fn is_running(&self) -> bool {
        self.is_running
    }

    /// Is the window currently focused?
    pub const fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Is the window being actively moved or resized?
    pub const fn is_changing(&self) -> bool {
        self.is_changing
    }
}
