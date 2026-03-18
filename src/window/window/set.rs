use crate::{
    Result,
    math::{Vector2i, Vector2u},
    window::Window,
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_title(title))
            .transpose()
            .map(|_| ())
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_position(position))
            .transpose()
            .map(|_| ())
    }

    /// Set the size of the client area of the window
    pub fn set_size(&mut self, size: Vector2i) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_size(size))
            .transpose()
            .map(|_| ())
    }

    /// Set the minimum size of the client area of the window
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_minimum_size(minimum_size))
            .transpose()
            .map(|_| ())
    }

    /// Set the maximum size of the client area of the window
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> Result<()> {
        self.with_inner_mut(|inner| inner.set_maximum_size(maximum_size))
            .transpose()
            .map(|_| ())
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        self.with_inner(|inner| inner.close())
            .transpose()
            .map(|_| ())
    }

    /// Maximize the window
    pub fn maximize(&mut self) -> Result<()> {
        self.with_inner_mut(|inner| inner.maximize())
            .transpose()
            .map(|_| ())
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        self.with_inner_mut(|inner| inner.minimize())
            .transpose()
            .map(|_| ())
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        self.with_inner_mut(|inner| inner.hide())
            .transpose()
            .map(|_| ())
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        self.with_inner_mut(|inner| inner.show())
            .transpose()
            .map(|_| ())
    }
}
