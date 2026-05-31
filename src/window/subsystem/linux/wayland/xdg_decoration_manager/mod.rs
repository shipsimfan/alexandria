use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::xdg_decoration::zxdg_decoration_manager_v1;

mod bind;
mod drop;
mod get;
mod get_top_level_decoration;

/// A wrapper around the `zxdg_decoration_manager_v1` global
pub(in crate::window) struct XdgDecorationManager {
    /// The raw handle to the `zxdg_decoration_manager_v1` global
    handle: *mut zxdg_decoration_manager_v1,

    /// The global name of this manager
    name: u32,

    /// The display this manager came from
    connection: Rc<WlDisplay>,
}
