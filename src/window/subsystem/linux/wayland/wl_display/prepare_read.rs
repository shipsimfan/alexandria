use crate::window::WlDisplay;

impl WlDisplay {
    /// Prepare this display for reading events from the underlying socket. If this function
    /// returns `false`, there are still events waiting to be dispatched. If this function returns
    /// `true`, either `read_events` or `cancel_read` must be called before any other Wayland
    /// functions.
    pub(in crate::window::subsystem::linux::wayland) fn prepare_read(&self) -> bool {
        let handle = self.handle.borrow_mut();

        unsafe { (self.library.f.display_prepare_read)(*handle) == 0 }
    }
}
