use crate::{
    math::{Recti, Vector2u},
    window::window::WindowInner,
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the position and size of the window's client area, in pixels
    pub fn rect(&self) -> Recti {
        self.window.rect()
    }

    /// Get the minimum size of the window's client area, in pixels
    pub fn minimum_size(&self) -> Option<Vector2u> {
        self.window.minimum_size()
    }

    /// Get the maximum size of the window's client area, in pixels
    pub fn maximum_size(&self) -> Option<Vector2u> {
        self.window.maximum_size()
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        self.window.is_fullscreen()
    }

    /// Is the window currently maximized?
    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    /// Is the window currently minimized?
    pub fn is_minimized(&self) -> bool {
        self.window.is_minimized()
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        self.window.is_focused()
    }
}
