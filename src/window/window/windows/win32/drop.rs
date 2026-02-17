use crate::window::{Win32Window, WindowProc};
use win32::{DestroyWindow, GWLP_USERDATA, SetWindowLongPtr, try_hresult};

impl<T: WindowProc> Drop for Win32Window<T> {
    fn drop(&mut self) {
        unsafe { SetWindowLongPtr(self.handle, GWLP_USERDATA, 0) };
        try_hresult!(DestroyWindow(self.handle)).unwrap();
    }
}
