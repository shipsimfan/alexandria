use crate::platform::linux::wayland::WlSurface;
use wayland::wl_surface;

impl WlSurface {
    /// Get the underlying handle to this surface
    pub(in crate::platform::linux::wayland) unsafe fn handle(&self) -> *mut wl_surface {
        self.handle
    }
}
