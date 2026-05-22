use crate::{
    PackedMap, Result,
    math::{Vector2i, Vector2u},
    window::{display::DisplayInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        todo!()
    }

    /// Set the position of the window
    pub fn set_position(
        &mut self,
        position: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        todo!()
    }

    /// Set the size of the client area of the window
    pub fn set_size(
        &mut self,
        size: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        todo!()
    }

    /// Set the minimum size of the client area of the window
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> Result<()> {
        todo!()
    }

    /// Set the maximum size of the client area of the window
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> Result<()> {
        todo!()
    }

    /// Send a close request to the window
    pub fn close(&self) -> Result<()> {
        todo!()
    }

    /// Maximize the window
    pub fn maximize(&mut self) -> Result<()> {
        todo!()
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        todo!()
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        todo!()
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        todo!()
    }

    /// Set if the window is borderless
    pub fn set_borderless(&mut self, borderless: bool) -> Result<()> {
        todo!()
    }

    /// Set if the window is resizable
    pub fn set_resizable(&mut self, resizable: bool) -> Result<()> {
        todo!()
    }

    /// Set if the window is fullscreen
    pub fn set_fullscreen(
        &mut self,
        fullscreen: bool,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        todo!()
    }
}
