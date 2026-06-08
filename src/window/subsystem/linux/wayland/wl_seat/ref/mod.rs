use crate::window::WlDisplay;
use std::{marker::PhantomData, ptr::NonNull, rc::Rc};
use wayland::wl_seat;

mod get_keyboard;
mod new;

/// A temporary reference to an [`WlSeat`](crate::window::WlSeat)
pub(in crate::window) struct WlSeatRef<'a, T> {
    /// The raw handle to the seat
    handle: *mut wl_seat,

    /// The connection this seat comes from
    connection: &'a Rc<WlDisplay>,

    /// The data used for listener callbacks
    listener_data: NonNull<(T, Rc<WlDisplay>)>,

    /// A marker for the listener data type, if any
    _phantom: PhantomData<T>,
}
