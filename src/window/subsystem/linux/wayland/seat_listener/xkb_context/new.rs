use crate::{Error, Result, window::subsystem::linux::wayland::seat_listener::XkbContext};
use xkbcommon::{XkbContextFlags, xkb_context_new};

impl XkbContext {
    /// Create a new [`XkbContext`]
    pub fn new() -> Result<XkbContext> {
        let handle = unsafe { xkb_context_new(XkbContextFlags::NoDefaultIncludes) };
        if handle.is_null() {
            return Err(Error::new("Failed to create xkb context"));
        }

        Ok(XkbContext { handle })
    }
}
