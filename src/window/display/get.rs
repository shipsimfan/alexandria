use crate::{
    math::{Rational, Recti, Vector2f, Vector2i, Vector2u},
    window::{Display, DisplayMode, DisplayOrientation},
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

    /// Get the current refresh rate
    pub fn refresh_rate(&self) -> Rational {
        self.inner().refresh_rate()
    }

    /// Get the DPI to use for UI scaling. 96 represents 100% scaling
    pub fn dpi(&self) -> u32 {
        self.inner().dpi()
    }

    /// Get the scale factor for UI
    pub fn content_scale(&self) -> f32 {
        (self.dpi() as f32) / 96.0
    }

    /// Get the physical size of this display, in millimeters
    pub fn physical_size(&self) -> Option<Vector2u> {
        self.inner().physical_size()
    }

    /// Get the physical size of this display, in inches
    pub fn physical_size_inches(&self) -> Option<Vector2f> {
        self.inner()
            .physical_size()
            .map(|physical_size| physical_size.into_f32() / 25.4)
    }

    /// Get the physical size of the diagonal of this display, in millimeters
    pub fn physical_diagonal(&self) -> Option<f32> {
        self.physical_size()
            .map(|physical_size| physical_size.into_f32().length())
    }

    /// Get the physical size of the diagonal of this display, in inches
    pub fn physical_diagonal_inches(&self) -> Option<f32> {
        self.physical_size_inches()
            .map(|physical_size| physical_size.into_f32().length())
    }

    /// Get the current orientation of the display
    pub fn current_orientation(&self) -> DisplayOrientation {
        self.inner().current_orientation()
    }

    /// Get the list of modes this display supports
    pub fn modes(&self) -> &[DisplayMode] {
        self.inner().modes()
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
