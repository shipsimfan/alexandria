use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::wl_surface;

mod commit;
mod drop;
mod get;
mod new;

/// A surface created from a Wayland compositor
pub(in crate::window) struct WlSurface {
    /// The handle to the underlying surface
    handle: *mut wl_surface,

    /// The display this surface came from
    connection: Rc<WlDisplay>,
}
