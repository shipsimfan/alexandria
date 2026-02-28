use crate::{
    math::{Rational, Recti, Vector2u},
    window::{DisplayOrientation, display::linux::wayland::WaylandDisplayEventHandler},
};

impl<UserEvent: 'static + Send> WaylandDisplayEventHandler<UserEvent> {
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

    /// Get the scale factor for UI
    pub fn content_scale(&self) -> f32 {
        self.content_scale
    }

    /// Get the physical of the display in millimeters
    pub fn physical_size(&self) -> Option<Vector2u> {
        self.physical_size
    }

    /// Get the current orientation of the display
    pub fn current_orientation(&self) -> DisplayOrientation {
        self.orientation
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
