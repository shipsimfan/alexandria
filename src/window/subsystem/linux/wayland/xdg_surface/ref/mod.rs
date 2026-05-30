use crate::window::XdgWmBase;
use std::rc::Rc;
use wayland::xdg_shell::xdg_surface;

mod ack_configure;
mod new;
mod set_window_geometry;

/// A temporary reference to an [`XdgSurface`](crate::window::XdgSurface)
pub(in crate::window) struct XdgSurfaceRef<'a> {
    /// The raw handle to the surface
    handle: *mut xdg_surface,

    /// The window manager this surface comes from
    wm: &'a Rc<XdgWmBase>,
}
