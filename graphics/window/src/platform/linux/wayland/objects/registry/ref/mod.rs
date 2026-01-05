use std::marker::PhantomData;
use wayland::wl_registry;

mod new;

/// A temporary reference to the global Wayland registry
pub(in crate::platform::linux::wayland) struct WlRegistryRef<'a> {
    /// The raw handle to the registry
    handle: *mut wl_registry,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'a ()>,
}
