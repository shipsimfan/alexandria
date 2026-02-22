use crate::window::subsystem::linux::WlDisplay;

impl WlDisplay {
    /// Cancel a previous `prepare_read`
    pub(in crate::window::subsystem::linux::wayland) fn cancel_read(&self) {
        let handle = self.handle.borrow_mut();
        unsafe { (self.library.f.display_cancel_read)(*handle) };
    }
}
