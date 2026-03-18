use crate::{
    Error, Result,
    math::{Rect, Vector2, Vector2i, Vector2u},
    window::window::WindowInner,
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.window.set_title(title)?;
        self.title = title.to_owned();
        Ok(())
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        let position = self
            .window
            .style()
            .client_to_window(Rect::new(position, self.rect().size))
            .map_err(|error| Error::new_with("unable to set a window's position", error))?
            .position;

        self.window.set_position(position)
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

    /// Set the minimum size of the client area of the window
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> Result<()> {
        self.window.set_minimum_size(minimum_size)
    }

    /// Set the maximum size of the client area of the window
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> Result<()> {
        self.window.set_maximum_size(maximum_size)
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        self.window.close()
    }

    /// Maximize the window
    pub fn maximize(&mut self) -> Result<()> {
        self.window.maximize()
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        self.window.minimize()
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        self.window.hide()
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        self.window.show()
    }
}
