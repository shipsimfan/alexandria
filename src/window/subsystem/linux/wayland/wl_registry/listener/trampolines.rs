use crate::window::subsystem::linux::wayland::{
    WlDisplay, WlRegistry, WlRegistryListener, WlRegistryRef,
};
use std::{
    ffi::{CStr, c_char, c_void},
    rc::Rc,
};
use wayland::{wl_registry, wl_registry_listener};

impl<T: WlRegistryListener> WlRegistry<T> {
    /// The listeners for the registry
    pub(in crate::window::subsystem::linux::wayland::wl_registry::listener) const LISTENER:
        wl_registry_listener = wl_registry_listener {
        global: global_trampoline::<T>,
        global_remove: global_remove_trampoline,
    };
}

/// Trampoline for responding to newly added global
unsafe extern "C" fn global_trampoline<T: WlRegistryListener>(
    data: *mut c_void,
    registry: *mut wl_registry,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    let data: &mut (T, Rc<WlDisplay>) = unsafe { &mut *data.cast() };

    let registry = WlRegistryRef::new(registry, &data.1);
    let interface = unsafe { CStr::from_ptr(interface) };

    data.0.global(registry, name, interface, version);
}

/// Trampoline for responding to a global being removed from the registry
unsafe extern "C" fn global_remove_trampoline(
    _: *mut c_void,
    _: *mut wayland::wl_registry,
    _: u32,
) {
}
