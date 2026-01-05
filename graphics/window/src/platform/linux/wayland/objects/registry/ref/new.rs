use crate::platform::linux::wayland::{WlDisplay, WlRegistryRef};
use std::{ptr::null_mut, rc::Rc};
use wayland::wl_registry;

impl<'a> WlRegistryRef<'a> {
    pub(in crate::platform::linux::wayland::objects::registry) fn new(
        handle: *mut wl_registry,
        display: &'a Rc<WlDisplay>,
    ) -> WlRegistryRef<'a> {
        assert_ne!(handle, null_mut());

        WlRegistryRef { handle, display }
    }
}
