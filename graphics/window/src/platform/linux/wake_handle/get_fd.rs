use crate::WindowWakeHandleInner;
use std::ffi::c_int;

impl WindowWakeHandleInner {
    /// Get the underlying file descriptor to the wake handle
    pub(in crate::platform::linux) fn get_fd(&self) -> c_int {
        self.event.handle()
    }
}
