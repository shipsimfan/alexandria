use crate::platform::linux::wayland::{XdgSurface, XdgToplevel, XdgWmBase};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_toplevel;

impl<T> XdgToplevel<T> {
    /// Create a new [`XdgToplevel`]
    pub(in crate::platform::linux::wayland::objects) fn new(
        handle: *mut xdg_toplevel,
        surface: XdgSurface<T>,
        wm: Rc<XdgWmBase>,
    ) -> XdgToplevel<T> {
        debug_assert_ne!(handle, null_mut());

        XdgToplevel {
            handle,
            surface,
            wm,
        }
    }
}
