use crate::window::{WlDisplay, XdgOutputManager};

impl XdgOutputManager {
    /// Get the name of this manager in the global registry
    pub(in crate::window::subsystem::linux::wayland) fn name(&self) -> u32 {
        self.name
    }

    /// Get the connection this manager came from
    pub fn connection(&self) -> &WlDisplay {
        &self.connection
    }
}
