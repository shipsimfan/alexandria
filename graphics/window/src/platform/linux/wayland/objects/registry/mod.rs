use crate::platform::linux::wayland::WlDisplay;
use std::{ptr::NonNull, rc::Rc};
use wayland::wl_registry;

mod listener;
mod r#ref;

mod data;
mod deref;
mod drop;
mod new;

pub(in crate::platform::linux::wayland) use listener::WlRegistryListener;
pub(in crate::platform::linux::wayland) use r#ref::WlRegistryRef;

/// The Wayland global registry
pub(in crate::platform::linux::wayland) struct WlRegistry<T = ()> {
    /// The raw handle to registry
    handle: *mut wl_registry,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<T>>,

    /// The display this registry came from
    display: Rc<WlDisplay>,
}
