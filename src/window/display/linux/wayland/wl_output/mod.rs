use crate::window::WlDisplay;
use std::{ptr::NonNull, rc::Rc};
use wayland::wl_output;

mod listener;

mod bind;
mod data;
mod deref;
mod drop;

pub(in crate::window::display::linux::wayland) use listener::WlOutputListener;

/// A Wayland output
pub(in crate::window::display::linux::wayland) struct WlOutput<T = ()> {
    /// The handle to the underlying Wayland output
    handle: *mut wl_output,

    /// Should this output be released when dropped?
    drop: bool,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<T>>,

    /// The display this output came from
    connection: Rc<WlDisplay>,
}
