use crate::Window;
use std::ptr::null_mut;
use win32::{GetModuleHandle, HINSTANCE, HWND};

impl Window {
    /// Get the handle for creating surfaces
    pub unsafe fn surface_creation_handle(&mut self) -> (HINSTANCE, HWND) {
        (unsafe { GetModuleHandle(null_mut()) }, *self.handle)
    }
}
