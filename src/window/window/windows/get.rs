use crate::{
    math::{Recti, Vector2u},
    window::window::WindowInner,
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the current title of the window
    pub fn title(&self) -> &str {
        &self.title
    }

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

    /// Get the current content scale factor of the window
    pub fn content_scale(&self) -> f32 {
        self.window.content_scale()
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

    /// Is the window currently visible?
    pub fn is_visible(&self) -> bool {
        self.window.is_visible()
    }

    /// Is the window borderless?
    pub fn is_borderless(&self) -> bool {
        self.is_borderless
    }

    /// Is the window resizable?
    pub fn is_resizable(&self) -> bool {
        self.is_resizable
    }
}
