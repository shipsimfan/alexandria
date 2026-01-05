use crate::platform::linux::wayland::{WaylandBind, WlCompositor, WlDisplay};
use std::rc::Rc;
use wayland::{wl_compositor, wl_compositor_interface, wl_interface};

impl WaylandBind for WlCompositor {
    type Handle = wl_compositor;

    const INTERFACE: &wl_interface = &wl_compositor_interface;

    fn from_handle(handle: *mut Self::Handle, display: Rc<WlDisplay>) -> Self {
        WlCompositor { handle, display }
    }
}
