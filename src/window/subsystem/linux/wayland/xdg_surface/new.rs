use crate::window::{WlSurface, XdgSurface, XdgWmBase};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_surface;

impl XdgSurface {
    /// Create a new [`XdgSurface`]
    pub(in crate::window::subsystem::linux::wayland) fn new(
        handle: *mut xdg_surface,
        surface: WlSurface,
        wm: Rc<XdgWmBase>,
    ) -> XdgSurface {
        debug_assert_ne!(handle, null_mut());

        XdgSurface {
            handle,
            surface: Some(surface),
            listener_data: None,
            wm,
        }
    }
}
