use crate::window::{WaylandBind, WlDisplay, display::linux::wayland::WlOutput};
use std::rc::Rc;
use wayland::{wl_interface, wl_output, wl_output_interface};

impl WaylandBind for WlOutput {
    type Handle = wl_output;

    const INTERFACE: &wl_interface = &wl_output_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>) -> Self {
        WlOutput {
            handle,
            drop: true,
            listener_data: None,
            connection,
        }
    }
}
