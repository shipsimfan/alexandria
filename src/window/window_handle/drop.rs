use crate::window::WindowHandle;
use win32::{try_hresult, DestroyWindow};

impl Drop for WindowHandle {
    fn drop(&mut self) {
        try_hresult!(DestroyWindow(self.handle)).unwrap();
    }
}
