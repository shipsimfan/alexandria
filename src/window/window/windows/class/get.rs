use crate::window::{WindowClass, WindowProc};
use win32::ATOM;

impl<T: WindowProc> WindowClass<T> {
    /// Get the handle to this [`WindowClass`]
    pub(in crate::window::window::windows) unsafe fn handle(&self) -> ATOM {
        self.handle
    }
}
