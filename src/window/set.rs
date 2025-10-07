use crate::{
    math::{Vector2i, Vector2u},
    Result, Window,
};

impl Window {
    /// Quit the application at the end of the next frame
    pub fn quit(&mut self) {
        self.is_running = false;
    }

    /// Set the windows size and position
    pub fn set_size_and_position(&mut self, size: Vector2u, position: Vector2i) -> Result<()> {
        self.handle.set_size_and_position(size, position)?;
        self.size = size;
        self.position = position;
        Ok(())
    }

    /// Set the size of the window
    pub fn set_size(&mut self, size: Vector2u) -> Result<()> {
        self.set_size_and_position(size, self.position)
    }

    /// Set the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        self.set_size_and_position(self.size, position)
    }
}
