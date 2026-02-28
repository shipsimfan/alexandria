use crate::{
    math::{Rational, Recti, Vector2u},
    window::{DisplayOrientation, display::linux::WaylandDisplay},
};

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Get the rectangle describing the area of this display covers
    pub fn rect(&self) -> Recti {
        self.output.rect()
    }

    /// Get the rectangle describing the work area of this display
    pub fn work_area(&self) -> Recti {
        self.output.work_area()
    }

    /// Get the current refresh rate
    pub fn refresh_rate(&self) -> Rational {
        self.output.refresh_rate()
    }

    /// Get the DPI to use for UI scaling. 96 represents 100% scaling
    pub fn dpi(&self) -> u32 {
        self.output.dpi()
    }

    /// Get the physical of the display in millimeters
    pub fn physical_size(&self) -> Option<Vector2u> {
        self.output.physical_size()
    }

    /// Get the current orientation of the display
    pub fn current_orientation(&self) -> DisplayOrientation {
        self.output.current_orientation()
    }

    /// Is this display the primary display?
    pub fn is_primary(&self) -> bool {
        self.output.is_primary()
    }

    /// Get the name of this display
    pub fn name(&self) -> &str {
        &self.output.name()
    }

    /// Get the id of this display
    pub fn id(&self) -> &str {
        &self.output.id()
    }
}
