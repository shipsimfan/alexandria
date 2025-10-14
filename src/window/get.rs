use crate::{
    graphics::GraphicsContext,
    math::{Vector2i, Vector2u},
    DisplayMode, Window,
};

impl Window {
    /// Is the window still running?
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get the x position relative to the screen of the lef side of the client area
    pub fn x(&self) -> i32 {
        self.position.x
    }

    /// Get the y position relative to the screen of the top of the client area
    pub fn y(&self) -> i32 {
        self.position.y
    }

    /// Get the position of the upper-left corner of the client area of the window
    pub fn position(&self) -> Vector2i {
        self.position
    }

    /// Get the width of the window's client area
    pub fn width(&self) -> u32 {
        self.size.x
    }

    /// Get the height of the window's client area
    pub fn height(&self) -> u32 {
        self.size.y
    }

    /// Get the size of the window's client area
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Is vertical sync enabled?
    pub fn vsync(&self) -> bool {
        self.vsync
    }

    /// Get the current display mode
    pub fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Get the graphics context for object creation
    pub fn graphics_context(&self) -> &GraphicsContext {
        &self.graphics_context
    }
}
