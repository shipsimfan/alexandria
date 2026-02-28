use crate::window::{WaylandBind, WlDisplay, XdgOutputManager};
use std::rc::Rc;
use wayland::xdg_output::{zxdg_output_manager_v1, zxdg_output_manager_v1_interface};

impl WaylandBind for XdgOutputManager {
    type Handle = zxdg_output_manager_v1;

    const INTERFACE: &'static wayland::wl_interface = &zxdg_output_manager_v1_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        XdgOutputManager {
            handle,
            name,
            connection,
        }
    }
}
