use crate::{
    Error, Result,
    math::{Rect, Vector2, Vector2i, Vector2u, number::Zero},
    window::window::WindowInner,
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.window.set_title(title)?;
        self.title = title.to_owned();
        Ok(())
    }

    /// Set the size of the client area of the window
    pub fn set_size(&mut self, size: Vector2i) -> Result<()> {
        let size = self
            .window
            .style()
            .client_to_window(Rect::new(Vector2::ZERO, size))
            .map_err(|error| Error::new_with("unable to set a window's size", error))?
            .size;

        self.window.set_size(size)?;
        Ok(())
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        self.window.close()
    }
}
