use crate::window::{XdgSurfaceRef, XdgWmBase};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_surface;

impl<'a> XdgSurfaceRef<'a> {
    /// Create a new reference to an [`XdgSurface`](crate::window::XdgSurface) from a raw handle
    pub(in crate::window::subsystem::linux::wayland::xdg_surface) fn new(
        handle: *mut xdg_surface,
        wm: &'a Rc<XdgWmBase>,
    ) -> XdgSurfaceRef<'a> {
        assert_ne!(handle, null_mut());

        XdgSurfaceRef { handle, wm }
    }
}
