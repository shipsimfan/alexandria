use crate::window::WlDisplay;
use std::{ptr::NonNull, rc::Rc};
use wayland::wl_registry;

mod listener;
mod r#ref;

mod data;
mod deref;
mod drop;
mod new;

pub(in crate::window) use r#ref::{WaylandBind, WlRegistryRef};

pub(in crate::window::subsystem::linux::wayland) use listener::WlRegistryListener;

/// The Wayland global registry
pub(in crate::window::subsystem::linux::wayland) struct WlRegistry<T = ()> {
    /// The raw handle to registry
    handle: *mut wl_registry,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<(T, Rc<WlDisplay>)>>,

    /// The display this registry came from
    connection: Rc<WlDisplay>,
}
