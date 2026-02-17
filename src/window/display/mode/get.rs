use crate::window::DisplayMode;

impl DisplayMode {
    /// Get the width of this mode
    pub fn width(&self) -> u32 {
        self.size.x
    }

    /// Get the height of this mode
    pub fn height(&self) -> u32 {
        self.size.y
    }
}
