use crate::platform::linux::wayland::{WaylandBind, WlDisplay, XdgWmBase};
use std::rc::Rc;
use wayland::{
    wl_interface,
    xdg_shell::{xdg_wm_base, xdg_wm_base_interface},
};

impl WaylandBind for XdgWmBase {
    type Handle = xdg_wm_base;

    const INTERFACE: &wl_interface = &xdg_wm_base_interface;

    fn from_handle(handle: *mut Self::Handle, display: Rc<WlDisplay>) -> Self {
        XdgWmBase { handle, display }
    }
}
