use crate::window::WlDisplay;
use std::{ptr::NonNull, rc::Rc};
use wayland::wl_output;

mod listener;

mod bind;
mod clone;
mod data;
mod deref;
mod drop;
mod get;

pub(in crate::window::display::linux::wayland) use listener::WlOutputListener;

/// A Wayland output
pub(in crate::window) struct WlOutput<T = ()> {
    /// The handle to the underlying Wayland output
    handle: *mut wl_output,

    /// The name of this output in the registry
    name: u32,

    /// Should this output be released when dropped?
    drop: bool,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<T>>,

    /// The display this output came from
    connection: Rc<WlDisplay>,
}
