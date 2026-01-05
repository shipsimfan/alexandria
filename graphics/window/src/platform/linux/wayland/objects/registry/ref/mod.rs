use crate::platform::linux::wayland::WlDisplay;
use std::rc::Rc;
use wayland::wl_registry;

mod bind;
mod new;

pub(in crate::platform::linux::wayland) use bind::WaylandBind;

/// A temporary reference to the global Wayland registry
pub(in crate::platform::linux::wayland) struct WlRegistryRef<'a> {
    /// The raw handle to the registry
    handle: *mut wl_registry,

    /// The display this registry comes from
    display: &'a Rc<WlDisplay>,
}
