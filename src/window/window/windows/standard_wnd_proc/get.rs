use crate::{math::Recti, window::StandardWndProc};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    pub(in crate::window::window::windows) fn rect(&self) -> Recti {
        self.rect
    }

    /// Is the window currently in fullscreen mode?
    pub(in crate::window::window::windows) fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}
