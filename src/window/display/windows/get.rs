use crate::{
    math::{Rational, Recti, Vector2u},
    window::{DisplayMode, DisplayOrientation, display::DisplayInner},
};

impl<UserEvent> DisplayInner<UserEvent> {
    /// Get the rectangle describing the area of this display covers
    pub fn rect(&self) -> Recti {
        self.rect
    }

    /// Get the rectangle describing the work area of this display
    pub fn work_area(&self) -> Recti {
        self.work_area
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
