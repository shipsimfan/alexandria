use crate::{
    math::{Vector2i, Vector2u},
    DisplayMode, Result, Window,
};

impl Window {
    /// Quit the application at the end of the next frame
    pub fn quit(&mut self) {
        self.is_running = false;
    }

    /// Set the window's display mode, size, and position
    pub fn set_display_mode_size_and_position(
        &mut self,
        display_mode: DisplayMode,
        size: Vector2u,
        position: Vector2i,
    ) -> Result<()> {
        self.handle
            .set_display_mode_size_and_position(display_mode, size, position)?;
        self.display_mode = display_mode;
        self.size = size;
        self.position = position;
        Ok(())
    }

    /// Set the window's size and position
    pub fn set_size_and_position(&mut self, size: Vector2u, position: Vector2i) -> Result<()> {
        self.set_display_mode_size_and_position(self.display_mode, size, position)
    }

    /// Set the size of the window
    pub fn set_size(&mut self, size: Vector2u) -> Result<()> {
        self.set_size_and_position(size, self.position)
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        self.set_size_and_position(self.size, position)
    }

    /// Set the mode the window should display as
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) -> Result<()> {
        self.set_display_mode_size_and_position(display_mode, self.size, self.position)
    }

    /// Sets if the rendering will be aligned with vertical syncs
    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    /// Set the window title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let mut utf16_title: Vec<_> = title.encode_utf16().collect();
        utf16_title.push(0);
        self.handle.set_title(&utf16_title)
    }
}
