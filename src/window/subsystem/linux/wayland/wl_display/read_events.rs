use crate::{Error, Result, window::subsystem::linux::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Actively read events from the Wayland socket after a prepared read
    pub(in crate::window::subsystem::linux::wayland) fn read_events(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_read_events)(*handle) } == 0 {
            return Ok(());
        }

        let errno = unsafe { *errno() };
        let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
        if error == 0 {
            error = errno;
        }

        Err(Error::new_with(
            "unable to read Wayland events",
            linux::Error::new(error),
        ))
    }
}
