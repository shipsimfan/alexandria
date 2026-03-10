use crate::{Result, window::Window};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_title(title))
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
