use crate::platform::linux::wayland::WlDisplay;
use std::rc::Rc;
use wayland::wl_surface;

mod drop;
mod new;

/// A surface created from a Wayland compositor
pub(in crate::platform::linux::wayland) struct WlSurface {
    /// The handle to the underlying surface
    handle: *mut wl_surface,

    /// The display that this surface comes from
    display: Rc<WlDisplay>,
}
