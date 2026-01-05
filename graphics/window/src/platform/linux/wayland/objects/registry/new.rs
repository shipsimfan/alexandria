use crate::platform::linux::wayland::{WlDisplay, WlRegistry};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_registry;

impl WlRegistry {
    /// Create a new [`WlRegistry`]
    pub(in crate::platform::linux::wayland::objects) fn new(
        handle: *mut wl_registry,
        display: Rc<WlDisplay>,
    ) -> WlRegistry {
        assert_ne!(handle, null_mut());

        WlRegistry {
            handle,
            listener_data: None,
            display,
        }
    }
}
