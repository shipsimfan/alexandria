use crate::window::{WlDisplay, WlSeatRef};
use std::{
    marker::PhantomData,
    ptr::{NonNull, null_mut},
    rc::Rc,
};
use wayland::wl_seat;

impl<'a, T> WlSeatRef<'a, T> {
    /// Create a new reference to an [`WlSeatRef`](crate::window::WlSeatRef) from a raw handle
    pub(in crate::window::subsystem::linux::wayland::objects::wl_seat) fn new(
        handle: *mut wl_seat,
        connection: &'a Rc<WlDisplay>,
        listener_data: NonNull<(T, Rc<WlDisplay>)>,
    ) -> WlSeatRef<'a, T> {
        assert_ne!(handle, null_mut());

        WlSeatRef {
            handle,
            connection,
            listener_data,
            _phantom: PhantomData,
        }
    }
}
