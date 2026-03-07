use crate::{
    math::{Recti, Vector2u},
    window::StandardWndProc,
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    pub(in crate::window::window::windows) fn rect(&self) -> Recti {
        self.rect
    }

    /// Get the minimum size of the window's client area, in pixels
    pub(in crate::window::window::windows) fn minimum_size(&self) -> Option<Vector2u> {
        self.minimum_client_size
    }

    /// Get the maximum size of the window's client area, in pixels
    pub(in crate::window::window::windows) fn maximum_size(&self) -> Option<Vector2u> {
        self.maximum_client_size
    }

    /// Is the window currently in fullscreen mode?
    pub(in crate::window::window::windows) fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    /// Is the window currently maximized?
    pub(in crate::window::window::windows) fn is_maximized(&self) -> bool {
        self.is_maximized
    }

    /// Is the window currently minimized?
    pub(in crate::window::window::windows) fn is_minimized(&self) -> bool {
        self.is_minimized
    }

    /// Is the window currently focused?
    pub(in crate::window::window::windows) fn is_focused(&self) -> bool {
        self.is_focused
    }
}
