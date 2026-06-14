use crate::{Error, Result, window::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Flush all waiting requests on the display
    pub(in crate::window::subsystem::linux::wayland) fn flush(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_flush)(*handle) } >= 0 {
            return Ok(());
        }

        let errno = unsafe { *errno() };
        let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
        if error == 0 {
            error = errno;
        }

        Err(Error::new_with(
            "unable to flush Wayland requests",
            linux::Error::new(error),
        ))
    }
}
