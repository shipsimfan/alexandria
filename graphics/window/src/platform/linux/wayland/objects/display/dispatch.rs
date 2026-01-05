use crate::{Result, WindowError, platform::linux::wayland::WlDisplay};
use linux::errno::errno;

impl WlDisplay {
    /// Dispatch all waiting events
    pub(in crate::platform::linux::wayland) fn dispatch(&self) -> Result<()> {
        let handle = self.handle.borrow_mut();
        if unsafe { (self.library.f.display_dispatch)(*handle) } < 0 {
            let errno = unsafe { *errno() };
            let mut error = unsafe { (self.library.f.display_get_error)(*handle) };
            if error == 0 {
                error = errno;
            }

            Err(WindowError::new_os(
                "unable to dispatch Wayland events",
                linux::Error::new(error),
            ))
        } else {
            Ok(())
        }
    }
}
