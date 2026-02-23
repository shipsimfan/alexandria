use crate::window::{WlDisplay, WlRegistryRef};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_registry;

impl<'a> WlRegistryRef<'a> {
    pub(in crate::window::subsystem::linux::wayland::wl_registry) fn new(
        handle: *mut wl_registry,
        connection: &'a Rc<WlDisplay>,
    ) -> WlRegistryRef<'a> {
        assert_ne!(handle, null_mut());

        WlRegistryRef { handle, connection }
    }
}
