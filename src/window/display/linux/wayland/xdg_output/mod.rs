use crate::window::{XdgOutputManager, display::linux::wayland::WlOutput};
use std::rc::Rc;
use wayland::xdg_output::zxdg_output_v1;

mod listener;

mod data;
mod downgrade;
mod drop;
mod get;
mod new;

pub(in crate::window) use listener::XdgOutputListener;

/// An XDG output
pub(in crate::window) struct XdgOutput<T = ()> {
    /// The handle to the underlying XDG output
    handle: *mut zxdg_output_v1,

    /// The output this XDG output is associated with
    wl_output: Option<WlOutput<T>>,

    /// The output manager this output came from
    manager: Rc<XdgOutputManager>,
}
