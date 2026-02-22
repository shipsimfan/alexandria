use crate::notify::NotifyInner;
use std::ffi::c_int;

impl NotifyInner {
    /// Get the file descriptor to the underlying event
    pub unsafe fn get_fd(&self) -> c_int {
        self.handle
    }
}
