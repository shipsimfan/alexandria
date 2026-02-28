use crate::{
    math::{Rational, Recti, Vector2u},
    window::{
        DisplayOrientation,
        display::linux::{WaylandDisplay, wayland::WaylandDisplayEventHandler},
    },
};

impl<UserEvent: 'static + Send> WaylandDisplay<UserEvent> {
    /// Get the rectangle describing the area of this display covers
    pub(in crate::window::display::linux) fn rect(&self) -> Recti {
        self.data().rect()
    }

    /// Get the rectangle describing the work area of this display
    pub(in crate::window::display::linux) fn work_area(&self) -> Recti {
        self.data().work_area()
    }

    /// Get the current refresh rate
    pub(in crate::window::display::linux) fn refresh_rate(&self) -> Rational {
        self.data().refresh_rate()
    }

    /// Get the scale factor for UI
    pub(in crate::window::display::linux) fn content_scale(&self) -> f32 {
        self.data().content_scale()
    }

    /// Get the physical of the display in millimeters
    pub(in crate::window::display::linux) fn physical_size(&self) -> Option<Vector2u> {
        self.data().physical_size()
    }

    /// Get the current orientation of the display
    pub(in crate::window::display::linux) fn current_orientation(&self) -> DisplayOrientation {
        self.data().current_orientation()
    }

    /// Is this display the primary display?
    pub(in crate::window::display::linux) fn is_primary(&self) -> bool {
        self.data().is_primary()
    }

    /// Get the name of this display
    pub(in crate::window::display::linux) fn name(&self) -> &str {
        self.data().name()
    }

    /// Get the id of this display
    pub(in crate::window::display::linux) fn id(&self) -> &str {
        &self.data().id()
    }

    /// Get the name of this display that the registry provided
    pub(in crate::window::display::linux) fn wayland_name(&self) -> u32 {
        match self {
            WaylandDisplay::Wl(output) => output.wayland_name(),
            WaylandDisplay::Xdg(output) => output.wayland_name(),
        }
    }

    /// Get a reference to the data of this display
    pub(in crate::window::display::linux::wayland) fn data(
        &self,
    ) -> &WaylandDisplayEventHandler<UserEvent> {
        match self {
            WaylandDisplay::Wl(output) => output.data(),
            WaylandDisplay::Xdg(output) => output.data(),
        }
    }

    /// Get a mutable reference to the data of this display
    pub(in crate::window::display::linux::wayland) fn data_mut(
        &mut self,
    ) -> &mut WaylandDisplayEventHandler<UserEvent> {
        match self {
            WaylandDisplay::Wl(output) => output.data_mut(),
            WaylandDisplay::Xdg(output) => output.data_mut(),
        }
    }
}
