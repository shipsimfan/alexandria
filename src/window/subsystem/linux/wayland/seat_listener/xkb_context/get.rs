use crate::window::subsystem::linux::wayland::seat_listener::XkbContext;

impl XkbContext {
    /// Get the handle to the xkb context
    pub fn handle(&self) -> *mut xkbcommon::XkbContext {
        self.handle
    }
}
