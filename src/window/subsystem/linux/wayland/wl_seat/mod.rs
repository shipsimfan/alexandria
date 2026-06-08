use crate::window::WlDisplay;
use std::{ptr::NonNull, rc::Rc};
use wayland::wl_seat;

mod listener;
mod r#ref;

mod bind;
mod drop;
mod get;

pub(in crate::window) use listener::WlSeatListener;
pub(in crate::window) use r#ref::WlSeatRef;

/// A seat represents a group of input devices
pub(in crate::window) struct WlSeat<T = ()> {
    /// The handle to the underlying Wayland seat
    handle: *mut wl_seat,

    /// The global name of this seat
    name: u32,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<(T, Rc<WlDisplay>)>>,

    /// The display this seat came from
    connection: Rc<WlDisplay>,
}
