use crate::window::{WlDisplay, subsystem::linux::wayland::WlRegistry};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_registry;

impl WlRegistry {
    /// Create a new [`WlRegistry`]
    pub fn new(handle: *mut wl_registry, connection: Rc<WlDisplay>) -> WlRegistry {
        assert_ne!(handle, null_mut());

        WlRegistry {
            handle,
            listener_data: None,
            connection,
        }
    }
}
