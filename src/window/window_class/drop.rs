use crate::window::WindowClass;
use std::ptr::null;
use win32::{try_get_last_error, GetModuleHandle, UnregisterClass};

impl Drop for WindowClass {
    fn drop(&mut self) {
        try_get_last_error!(unsafe { UnregisterClass(self.class as _, GetModuleHandle(null())) })
            .unwrap();
    }
}
