use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::xdg_shell::xdg_wm_base;

mod bind;
mod drop;
mod get;
mod get_xdg_surface;
mod register_ping_handler;

/// A wrapper around the `xdg_wm_base` global
pub(in crate::window) struct XdgWmBase {
    /// The raw handle to the `xdg_wm_base` global
    handle: *mut xdg_wm_base,

    /// The global name of this manager
    name: u32,

    /// The display this manager came from
    connection: Rc<WlDisplay>,
}
