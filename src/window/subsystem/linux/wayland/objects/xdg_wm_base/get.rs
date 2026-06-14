use crate::window::{WaylandLibrary, XdgWmBase};

impl XdgWmBase {
    /// Get the name of this manager in the global registry
    pub(in crate::window::subsystem::linux::wayland) fn name(&self) -> u32 {
        self.name
    }

    /// Get a reference to the [`WaylandLibrary`] that holds the functions for this window manager
    pub(in crate::window::subsystem::linux::wayland) fn library(&self) -> &WaylandLibrary {
        &self.connection.library
    }
}
