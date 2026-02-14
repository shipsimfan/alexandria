use crate::{
    math::{Recti, Vector2i},
    window::display::DisplayInner,
};

impl DisplayInner {
    /// Get the rectangle describing the area of this display covers
    pub fn rect(&self) -> Recti {
        self.rect
    }

    /// Get the position of this display
    pub fn position(&self) -> Vector2i {
        self.rect.position
    }

    /// Get the x-position of this display
    pub fn x(&self) -> i32 {
        self.rect.position.x
    }

    /// Get the y-position of this display
    pub fn y(&self) -> i32 {
        self.rect.position.y
    }

    /// Get the size of this display
    pub fn size(&self) -> Vector2i {
        self.rect.size
    }

    /// Get the width of this display
    pub fn width(&self) -> i32 {
        self.rect.size.x
    }

    /// Get the height of this display
    pub fn height(&self) -> i32 {
        self.rect.size.y
    }

    /// Get the rectangle describing the work area of this display
    pub fn work_area(&self) -> Recti {
        self.work_area
    }

    /// Get the position of the work area of this display
    pub fn work_area_position(&self) -> Vector2i {
        self.work_area.position
    }

    /// Get the x-position of the work area of this display
    pub fn work_area_x(&self) -> i32 {
        self.work_area.position.x
    }

    /// Get the y-position of the work area of this display
    pub fn work_area_y(&self) -> i32 {
        self.work_area.position.y
    }

    /// Get the size of the work area of this display
    pub fn work_area_size(&self) -> Vector2i {
        self.work_area.size
    }

    /// Get the width of the work area of this display
    pub fn work_area_width(&self) -> i32 {
        self.work_area.size.x
    }

    /// Get the height of the work area of this display
    pub fn work_area_height(&self) -> i32 {
        self.work_area.size.y
    }

    /// Get the DPI to use for UI scaling. 96 represents 100% scaling
    pub fn dpi(&self) -> u32 {
        self.dpi
    }

    /// Is this display the primary display?
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Get the name of this display
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the id of this display
    pub fn id(&self) -> &str {
        &self.id
    }
}
