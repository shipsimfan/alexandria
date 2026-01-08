use crate::platform::linux::wayland::{WlSurface, XdgSurface, XdgWmBase};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_surface;

impl XdgSurface {
    /// Create a new [`XdgSurface`]
    pub(in crate::platform::linux::wayland::objects) fn new(
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
