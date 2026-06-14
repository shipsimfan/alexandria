use crate::window::{WlDisplay, WlSurface};
use wayland::wl_surface;

impl WlSurface {
    /// Get the underlying handle to this surface
    pub(in crate::window::subsystem::linux::wayland) unsafe fn handle(&self) -> *mut wl_surface {
        self.handle
    }

    /// Get the connection this surface is associated with
    pub(in crate::window::subsystem::linux::wayland) fn connection(&self) -> &WlDisplay {
        &self.connection
    }
}
