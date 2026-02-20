use crate::{math::Recti, window::window::WindowInner};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        self.window.rect
    }
}
