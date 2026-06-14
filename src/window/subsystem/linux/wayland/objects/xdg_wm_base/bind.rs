use crate::window::{WaylandBind, WlDisplay, XdgWmBase};
use std::rc::Rc;
use wayland::xdg_shell::{xdg_wm_base, xdg_wm_base_interface};

impl WaylandBind for XdgWmBase {
    type Handle = xdg_wm_base;

    const INTERFACE: &'static wayland::wl_interface = &xdg_wm_base_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        XdgWmBase {
            handle,
            connection,
            name,
        }
    }
}
