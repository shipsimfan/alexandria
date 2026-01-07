use crate::platform::linux::wayland::WlDisplay;
use std::rc::Rc;
use wayland::wl_compositor;

mod bind;
mod create_surface;

/// A reference to the global Wayland compositor
pub(in crate::platform::linux::wayland) struct WlCompositor {
    /// The handle to the underlying compositor
    handle: *mut wl_compositor,

    /// The display this compositor came from
    display: Rc<WlDisplay>,
}
