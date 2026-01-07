use crate::platform::linux::wayland::{WlDisplay, WlSurface};
use std::rc::Rc;
use wayland::wl_surface;

impl WlSurface {
    /// Create a new [`WlSurface`]
    pub(in crate::platform::linux::wayland::objects) fn new(
        handle: *mut wl_surface,
        display: Rc<WlDisplay>,
    ) -> WlSurface {
        WlSurface { handle, display }
    }
}
