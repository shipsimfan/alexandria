use crate::{Result, WindowError, platform::linux::wayland::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Flush all waiting requests on the display
    pub(in crate::platform::linux::wayland) fn flush(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_flush)(*handle) } >= 0 {
            return Ok(());
        }

        let errno = unsafe { *errno() };
        let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
        if error == 0 {
            error = errno;
        }

        Err(WindowError::new_os(
            "unable to flush Wayland requests",
            linux::Error::new(error),
        ))
    }
}
