use crate::window::display::linux::wayland::WaylandDisplayEventHandler;

impl<UserEvent: 'static + Send> WaylandDisplayEventHandler<UserEvent> {
    /// Disable events for this display
    pub fn disable_events(&mut self) {
        self.events_enabled = false;
    }
}
