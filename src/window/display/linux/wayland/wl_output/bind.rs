use crate::window::{WaylandBind, WlDisplay, WlOutput};
use std::rc::Rc;
use wayland::{wl_interface, wl_output, wl_output_interface};

impl WaylandBind for WlOutput {
    type Handle = wl_output;

    const INTERFACE: &wl_interface = &wl_output_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        WlOutput {
            handle,
            name,
            drop: true,
            listener_data: None,
            connection,
        }
    }
}
