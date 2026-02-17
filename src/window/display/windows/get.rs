use crate::{
    math::{Rational, Recti, Vector2i, Vector2u},
    window::{DisplayMode, DisplayOrientation, display::DisplayInner},
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

    /// Get the current refresh rate
    pub fn refresh_rate(&self) -> Rational {
        self.refresh_rate
    }

    /// Get the DPI to use for UI scaling. 96 represents 100% scaling
    pub fn dpi(&self) -> u32 {
        self.dpi
    }

    /// Get the physical of the display in millimeters
    pub fn physical_size(&self) -> Option<Vector2u> {
        self.physical_size
    }

    /// Get the current orientation of the display
    pub fn current_orientation(&self) -> DisplayOrientation {
        self.orientation
    }

    /// Get the list of modes this display supports
    pub fn modes(&self) -> &[DisplayMode] {
        &self.modes
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
