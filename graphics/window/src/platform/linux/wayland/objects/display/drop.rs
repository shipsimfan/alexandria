use crate::platform::linux::wayland::WlDisplay;

impl Drop for WlDisplay {
    fn drop(&mut self) {
        unsafe { (self.library.f.display_disconnect)(self.handle) };
    }
}
