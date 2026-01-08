use crate::platform::linux::wayland::XdgWmBase;
use std::rc::Rc;
use wayland::xdg_shell::xdg_surface;

mod ack_configure;
mod new;

/// A temporary reference to an [`XdgSurface`](crate::platform::linux::wayland::XdgSurface)
pub(in crate::platform::linux::wayland) struct XdgSurfaceRef<'a> {
    /// The raw handle to the surface
    handle: *mut xdg_surface,

    /// The window manager this surface comes from
    wm: &'a Rc<XdgWmBase>,
}
