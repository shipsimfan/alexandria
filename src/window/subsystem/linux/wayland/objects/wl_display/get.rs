use crate::window::WlDisplay;
use wayland::wl_display;

impl WlDisplay {
    /// Get the underlying handle to this display
    pub(in crate::window::subsystem::linux::wayland) unsafe fn handle(&self) -> *mut wl_display {
        *self.handle.borrow()
    }
}
