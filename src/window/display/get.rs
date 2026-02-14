use crate::{
    math::{Recti, Vector2i},
    window::Display,
};

impl<'a> Display<'a> {
    /// Get the rectangle describing the area of this display covers
    pub fn rect(&self) -> Recti {
        self.inner().rect()
    }

    /// Get the position of this display
    pub fn position(&self) -> Vector2i {
        self.inner().position()
    }

    /// Get the x-position of this display
    pub fn x(&self) -> i32 {
        self.inner().x()
    }

    /// Get the y-position of this display
    pub fn y(&self) -> i32 {
        self.inner().y()
    }

    /// Get the size of this display
    pub fn size(&self) -> Vector2i {
        self.inner().size()
    }

    /// Get the width of this display
    pub fn width(&self) -> i32 {
        self.inner().width()
    }

    /// Get the height of this display
    pub fn height(&self) -> i32 {
        self.inner().height()
    }

    /// Get the rectangle describing the work area of this display
    pub fn work_area(&self) -> Recti {
        self.inner().work_area()
    }

    /// Get the position of the work area of this display
    pub fn work_area_position(&self) -> Vector2i {
        self.inner().work_area_position()
    }

    /// Get the x-position of the work area of this display
    pub fn work_area_x(&self) -> i32 {
        self.inner().work_area_x()
    }

    /// Get the y-position of the work area of this display
    pub fn work_area_y(&self) -> i32 {
        self.inner().work_area_y()
    }

    /// Get the size of the work area of this display
    pub fn work_area_size(&self) -> Vector2i {
        self.inner().work_area_size()
    }

    /// Get the width of the work area of this display
    pub fn work_area_width(&self) -> i32 {
        self.inner().work_area_width()
    }

    /// Get the height of the work area of this display
    pub fn work_area_height(&self) -> i32 {
        self.inner().work_area_height()
    }

    /// Get the DPI to use for UI scaling. 96 represents 100% scaling
    pub fn dpi(&self) -> u32 {
        self.inner().dpi()
    }

    /// Get the scale factor for UI
    pub fn content_scale(&self) -> f32 {
        (self.dpi() as f32) / 96.0
    }

    /// Is this display the primary display?
    pub fn is_primary(&self) -> bool {
        self.inner().is_primary()
    }

    /// Get the name of this display
    pub fn name(&self) -> &str {
        self.inner().name()
    }

    /// Get the id of this display
    pub fn id(&self) -> &str {
        self.inner().id()
    }
}
