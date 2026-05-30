use crate::window::{WaylandBind, WlCompositor, WlDisplay};
use std::rc::Rc;
use wayland::{wl_compositor, wl_compositor_interface};

impl WaylandBind for WlCompositor {
    type Handle = wl_compositor;

    const INTERFACE: &'static wayland::wl_interface = &wl_compositor_interface;

    fn from_handle(handle: *mut Self::Handle, connection: Rc<WlDisplay>, name: u32) -> Self {
        WlCompositor {
            handle,
            connection,
            name,
        }
    }
}
