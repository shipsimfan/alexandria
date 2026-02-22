use crate::notify::NotifyInner;
use win32::HANDLE;

impl NotifyInner {
    /// Get the handle to the underlying event
    pub unsafe fn handle(&self) -> HANDLE {
        self.handle
    }
}
