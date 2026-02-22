use crate::window::subsystem::linux::{WlDisplay, wayland::WlRegistryRef};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_registry;

impl<'a> WlRegistryRef<'a> {
    pub(in crate::window::subsystem::linux::wayland::wl_registry) fn new(
        handle: *mut wl_registry,
        display: &'a Rc<WlDisplay>,
    ) -> WlRegistryRef<'a> {
        assert_ne!(handle, null_mut());

        WlRegistryRef { handle, display }
    }
}
