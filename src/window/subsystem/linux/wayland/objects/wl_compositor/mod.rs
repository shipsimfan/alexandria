use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::wl_compositor;

mod bind;
mod create_surface;
mod get;

/// A wrapper around the `wl_compositor` global
pub(in crate::window) struct WlCompositor {
    /// The raw handle to the `wl_compositor` global
    handle: *mut wl_compositor,

    /// The global name of this compositor
    name: u32,

    /// The display this compositor came from
    connection: Rc<WlDisplay>,
}
