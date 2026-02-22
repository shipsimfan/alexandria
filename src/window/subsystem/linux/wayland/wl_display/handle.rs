use crate::window::subsystem::linux::WlDisplay;
use wayland::wl_display;

impl WlDisplay {
    /// Get the underlying handle to this surface
    pub(in crate::window::subsystem::linux::wayland) unsafe fn handle(&self) -> *mut wl_display {
        *self.handle.borrow_mut()
    }
}
