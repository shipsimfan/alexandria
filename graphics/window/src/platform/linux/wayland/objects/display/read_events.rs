use crate::{Result, WindowError, platform::linux::wayland::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Actively read events from the Wayland socket after a prepared read
    pub(in crate::platform::linux::wayland) fn read_events(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_read_events)(*handle) } == 0 {
            return Ok(());
        }

        let errno = unsafe { *errno() };
        let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
        if error == 0 {
            error = errno;
        }

        Err(WindowError::new_os(
            "unable to read Wayland events",
            linux::Error::new(error),
        ))
    }
}
