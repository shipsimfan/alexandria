use crate::window::subsystem::linux::wayland::WaylandGlobals;

impl<UserEvent: 'static + Send> WaylandGlobals<UserEvent> {
    /// Enable events for this set of [`WaylandGlobals`]
    pub fn enable_events(&mut self) {
        assert!(!self.events_enabled);
        self.events_enabled = true;
    }
}
