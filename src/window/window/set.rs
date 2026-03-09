use crate::{Result, window::Window};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_title(title))
            .transpose()
            .map(|_| ())
    }
}
