use crate::{Result, window::window::WindowInner};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.window.set_title(title)?;
        self.title = title.to_owned();
        Ok(())
    }
}
