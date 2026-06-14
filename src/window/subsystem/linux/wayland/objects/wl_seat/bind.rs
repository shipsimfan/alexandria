use crate::window::{WaylandBind, WlDisplay, WlSeat};
use std::rc::Rc;
use wayland::{wl_seat, wl_seat_interface};

impl WaylandBind for WlSeat<()> {
    type Handle = wl_seat;

    const INTERFACE: &'static wayland::wl_interface = &wl_seat_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        WlSeat {
            handle,
            connection,
            name,
            listener_data: None,
        }
    }
}
