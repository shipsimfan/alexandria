use crate::notify::NotifyInner;
use std::os::windows::raw::HANDLE;

impl NotifyInner {
    /// Get the handle to the underlying event
    pub unsafe fn handle(&self) -> HANDLE {
        self.handle
    }
}
