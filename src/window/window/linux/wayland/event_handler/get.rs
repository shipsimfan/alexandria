use crate::{math::Recti, window::window::linux::wayland::WaylandEventHandler};

impl<UserEvent: 'static + Send> WaylandEventHandler<UserEvent> {
    /// Get the current position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        self.rect
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    /// Is the window currently maximized?
    pub fn is_maximized(&self) -> bool {
        self.is_maximized
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Is the window borderless?
    pub fn is_borderless(&self) -> bool {
        self.is_borderless
    }
}
