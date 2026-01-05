use crate::{Result, WindowError, platform::linux::wayland::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Block until all outstanding requests have been processed
    pub(in crate::platform::linux::wayland) fn roundtrip(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_roundtrip)(*handle) } >= 0 {
            return Ok(());
        }

        let errno = unsafe { *errno() };
        let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
        if error == 0 {
            error = errno;
        }

        Err(WindowError::new_os(
            "unable to process Wayland events",
            linux::Error::new(error),
        ))
    }
}
