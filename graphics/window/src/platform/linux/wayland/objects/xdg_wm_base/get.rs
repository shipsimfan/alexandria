use crate::platform::linux::wayland::{WaylandLibrary, XdgWmBase};

impl XdgWmBase {
    /// Get a reference to the [`WaylandLibrary`] that holds the functions for this window manager
    pub(in crate::platform::linux::wayland::objects) fn library(&self) -> &WaylandLibrary {
        &self.display.library
    }
}
