use crate::window::WlDisplay;
use std::ffi::c_int;

impl WlDisplay {
    /// Get the underlying file descriptor
    pub(in crate::window::subsystem::linux::wayland) fn get_fd(&self) -> c_int {
        let handle = self.handle.borrow_mut();
        unsafe { (self.library.f.display_get_fd)(*handle) }
    }
}
