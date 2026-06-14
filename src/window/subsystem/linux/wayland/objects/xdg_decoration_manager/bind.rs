use crate::window::{WaylandBind, WlDisplay, XdgDecorationManager};
use std::rc::Rc;
use wayland::xdg_decoration::{zxdg_decoration_manager_v1, zxdg_decoration_manager_v1_interface};

impl WaylandBind for XdgDecorationManager {
    type Handle = zxdg_decoration_manager_v1;

    const INTERFACE: &'static wayland::wl_interface = &zxdg_decoration_manager_v1_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        XdgDecorationManager {
            handle,
            connection,
            name,
        }
    }
}
