use crate::{
    Error, Result,
    window::subsystem::linux::{WlDisplay, wayland::WlRegistry},
};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_display_get_registry_dyn;

impl WlDisplay {
    /// Get the global Wayland registry
    pub(in crate::window::subsystem::linux::wayland) fn get_registry(
        self: Rc<Self>,
    ) -> Result<WlRegistry> {
        let handle = unsafe {
            wl_display_get_registry_dyn(
                *self.handle.borrow(),
                *self.library.f.proxy_marshal_flags,
                *self.library.f.proxy_get_version,
            )
        };
        if handle == null_mut() {
            Err(Error::new("unable to get Wayland registry"))
        } else {
            Ok(WlRegistry::new(handle, self))
        }
    }
}
