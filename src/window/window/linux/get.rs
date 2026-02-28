use crate::{math::Recti, window::window::WindowInner};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        todo!()
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        todo!()
    }
}
