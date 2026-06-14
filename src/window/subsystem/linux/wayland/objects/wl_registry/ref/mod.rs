use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::wl_registry;

mod bind;
mod new;

pub(in crate::window) use bind::WaylandBind;

/// A temporary reference to the global Wayland registry
pub(in crate::window) struct WlRegistryRef<'a> {
    /// The raw handle to the registry
    handle: *mut wl_registry,

    /// The display this registry comes from
    connection: &'a Rc<WlDisplay>,
}
