use crate::{
    math::{Recti, Vector2u},
    window::window::{WindowInner, WindowSurfaceCreationHandle},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the current title of the window
    pub fn title(&self) -> &str {
        todo!()
    }

    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        todo!()
    }

    /// Get the minimum size of the window's client area, in pixels
    pub fn minimum_size(&self) -> Option<Vector2u> {
        todo!()
    }

    /// Get the maximum size of the window's client area, in pixels
    pub fn maximum_size(&self) -> Option<Vector2u> {
        todo!()
    }

    /// Get the current content scale factor of the window
    pub fn content_scale(&self) -> f32 {
        todo!()
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        todo!()
    }

    /// Is the window currently maximized?
    pub fn is_maximized(&self) -> bool {
        todo!()
    }

    /// Is the window currently minimized?
    pub fn is_minimized(&self) -> bool {
        todo!()
    }

    /// Is the window currently focused?
    pub fn is_focused(&self) -> bool {
        todo!()
    }

    /// Is the window currently visible?
    pub fn is_visible(&self) -> bool {
        todo!()
    }

    /// Is the window borderless?
    pub fn is_borderless(&self) -> bool {
        todo!()
    }

    /// Is the window resizable?
    pub fn is_resizable(&self) -> bool {
        todo!()
    }

    /// Get the handle for creating a surface for this window
    pub fn surface_creation_handle(&self) -> WindowSurfaceCreationHandle {
        todo!()
    }
}
