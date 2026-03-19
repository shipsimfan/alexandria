use crate::{
    Error, PackedMap, Result,
    math::{Rect, Vector2, Vector2i, Vector2u},
    window::{display::DisplayInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Set the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.window.set_title(title)?;
        self.title = title.to_owned();
        Ok(())
    }

    /// Set the position of the window
    pub fn set_position(
        &mut self,
        position: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        let position = if self.window.is_fullscreen() {
            self.window.set_windowed_position(position);

            let mut set_position = None;
            for display in displays {
                if display.rect().contains_point(&position) {
                    set_position = Some(display.rect().position);
                    break;
                }
            }

            match set_position {
                Some(set_position) => set_position,
                None => return Ok(()),
            }
        } else {
            position
        };

        let position = self
            .window
            .style()
            .client_to_window(Rect::new(position, self.rect().size))
            .map_err(|error| Error::new_with("unable to set a window's position", error))?
            .position;

        self.window.set_position(position)
    }

    /// Set the size of the client area of the window
    pub fn set_size(
        &mut self,
        size: Vector2i,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        let size = if self.window.is_fullscreen() {
            self.window.set_windowed_size(size);

            let mut set_size = None;
            for display in displays {
                if display.rect().contains_point(&self.window.rect().position) {
                    set_size = Some(display.rect().size);
                    break;
                }
            }

            match set_size {
                Some(set_size) => set_size,
                None => return Ok(()),
            }
        } else {
            size
        };

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
        if self.window.is_fullscreen() {
            self.window.set_maximized();
            Ok(())
        } else {
            self.window.maximize()
        }
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        if self.window.is_fullscreen() {
            self.window.set_minimized();
            Ok(())
        } else {
            self.window.minimize()
        }
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        self.window.hide()
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        self.window.show()
    }

    /// Set if the window is borderless
    pub fn set_borderless(&mut self, borderless: bool) -> Result<()> {
        if let Some(style) = self.window.set_borderless(borderless) {
            self.window.set_style(style)?;
        }
        Ok(())
    }

    /// Set if the window is resizable
    pub fn set_resizable(&mut self, resizable: bool) -> Result<()> {
        if let Some(style) = self.window.set_resizable(resizable) {
            self.window.set_style(style)?;
        }
        Ok(())
    }
}
