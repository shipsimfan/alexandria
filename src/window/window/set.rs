use crate::{
    Result,
    math::{Vector2i, Vector2u},
    window::Window,
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.set_title(title))
            .transpose()
            .map(|_| ())
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        self.with_inner_mut(|inner, displays| inner.set_position(position, displays))
            .transpose()
            .map(|_| ())
    }

    /// Set the size of the client area of the window
    pub fn set_size(&mut self, size: Vector2i) -> Result<()> {
        self.with_inner_mut(|inner, displays| inner.set_size(size, displays))
            .transpose()
            .map(|_| ())
    }

    /// Set the minimum size of the client area of the window
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.set_minimum_size(minimum_size))
            .transpose()
            .map(|_| ())
    }

    /// Set the maximum size of the client area of the window
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.set_maximum_size(maximum_size))
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
        self.with_inner_mut(|inner, _| inner.maximize())
            .transpose()
            .map(|_| ())
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.minimize())
            .transpose()
            .map(|_| ())
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.hide())
            .transpose()
            .map(|_| ())
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.show())
            .transpose()
            .map(|_| ())
    }

    /// Set if the window is borderless
    pub fn set_borderless(&mut self, borderless: bool) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.set_borderless(borderless))
            .transpose()
            .map(|_| ())
    }

    /// Set if the window is resizable
    pub fn set_resizable(&mut self, resizable: bool) -> Result<()> {
        self.with_inner_mut(|inner, _| inner.set_resizable(resizable))
            .transpose()
            .map(|_| ())
    }
}
