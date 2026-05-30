use crate::window::{WlDisplay, WlSurface};
use std::rc::Rc;
use wayland::wl_surface;

impl WlSurface {
    /// Create a new [`WlSurface`]
    pub(in crate::window::subsystem::linux::wayland) fn new(
        handle: *mut wl_surface,
        connection: Rc<WlDisplay>,
    ) -> WlSurface {
        WlSurface { handle, connection }
    }
}
