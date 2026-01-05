use crate::platform::linux::wayland::WlRegistryRef;
use std::{marker::PhantomData, ptr::null_mut};
use wayland::wl_registry;

impl<'a> WlRegistryRef<'a> {
    pub(in crate::platform::linux::wayland::objects::registry) fn new(
        handle: *mut wl_registry,
    ) -> WlRegistryRef<'a> {
        assert_ne!(handle, null_mut());

        WlRegistryRef {
            handle,
            _lifetime: PhantomData,
        }
    }
}
