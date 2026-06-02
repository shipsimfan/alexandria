use crate::{
    Error, PackedMap, Result,
    math::{Rect, Vector2i, Vector2u},
    window::{WindowStyle, display::DisplayInner, window::WindowInner},
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
        size: Vector2u,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        let size = if self.window.is_fullscreen() {
            self.window.set_windowed_size(size);

            let mut set_size = None;
            for display in displays {
                if display.rect().contains_point(&self.window.rect().position) {
                    set_size = Some(Vector2u::new(
                        display.rect().size.x as _,
                        display.rect().size.y as _,
                    ));
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
            .client_to_window(Rect::new(
                Vector2i::ZERO,
                Vector2i::new(size.x as _, size.y as _),
            ))
            .map_err(|error| Error::new_with("unable to set a window's size", error))?
            .size;

        self.window
            .set_size(Vector2u::new(size.x as _, size.y as _))?;
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

    /// Set if the window is fullscreen
    pub fn set_fullscreen(
        &mut self,
        fullscreen: bool,
        displays: &PackedMap<DisplayInner<UserEvent>>,
    ) -> Result<()> {
        if fullscreen == self.is_fullscreen() {
            return Ok(());
        }

        self.window.set_fullscreen(fullscreen);

        if fullscreen {
            // Find a display to fullscreen on
            let position = self.window.rect().position;
            let mut found_display = None;
            for display in displays {
                if display.rect().contains_point(&position) {
                    found_display = Some(display);
                    break;
                }
            }
            let display = found_display.unwrap_or_else(|| displays.at_index(0).1);

            // Set the window style to fullscreen
            self.window.set_style(WindowStyle::fullscreen())?;

            // Set the window size and position to the display's
            self.window.set_position(display.rect().position)?;
            self.window.set_size(Vector2u::new(
                display.rect().size.x as _,
                display.rect().size.y as _,
            ))
        } else {
            // Set the window style to windowed
            let style = self.window.style();
            self.window.set_style(style)?;

            // Set the window size and position to the previous windowed size and position
            let rect = style
                .client_to_window(self.window.windowed_rect())
                .map_err(|os| Error::new_with("unable to set a window's size", os))?;

            self.window.set_position(rect.position)?;
            self.window
                .set_size(Vector2u::new(rect.size.x as _, rect.size.y as _))?;

            // Set the window maximized or minimized state to the previous windowed maximized or minimized state
            if self.window.is_maximized() {
                self.window.maximize()?;
            } else if self.window.is_minimized() {
                self.window.minimize()?;
            }

            Ok(())
        }
    }
}
