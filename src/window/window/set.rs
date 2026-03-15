use crate::{Result, math::Vector2i, window::Window};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_title(title))
            .transpose()
            .map(|_| ())
    }

    /// Set the size of the client area of the window
    pub fn set_size(&mut self, size: Vector2i) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_size(size))
            .transpose()
            .map(|_| ())
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        self.with_inner(|inner| inner.close())
            .transpose()
            .map(|_| ())
    }
}
