use crate::window::subsystem::linux::WlDisplay;

impl Drop for WlDisplay {
    fn drop(&mut self) {
        unsafe { (self.library.f.display_disconnect)(*self.handle.borrow()) };
    }
}
