use crate::window::{WindowClass, WindowProc};
use std::ptr::null;
use win32::{GetModuleHandle, UnregisterClass, try_get_last_error};

impl<T: WindowProc> Drop for WindowClass<T> {
    fn drop(&mut self) {
        try_get_last_error!(unsafe { UnregisterClass(self.handle as _, GetModuleHandle(null())) })
            .unwrap();
    }
}
