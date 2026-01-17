use crate::platform::linux::wayland::WlDisplay;
use wayland::wl_display;

impl WlDisplay {
    /// Get the underlying handle to this surface
    pub(in crate::platform::linux::wayland) unsafe fn handle(&self) -> *mut wl_display {
        *self.handle.borrow_mut()
    }
}
