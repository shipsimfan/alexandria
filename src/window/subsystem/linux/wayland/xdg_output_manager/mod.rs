use crate::window::WlDisplay;
use std::rc::Rc;
use wayland::xdg_output::zxdg_output_manager_v1;

mod bind;
mod drop;
mod get;
mod get_xdg_output;

pub(in crate::window) struct XdgOutputManager {
    /// The raw handle to the `zxdg_output_manager_v1` global
    handle: *mut zxdg_output_manager_v1,

    /// The global name of this manager
    name: u32,

    /// The display this manager came from
    connection: Rc<WlDisplay>,
}
