use crate::window::{XdgDecorationManager, XdgTopLevel};
use std::rc::Rc;
use wayland::xdg_decoration::zxdg_toplevel_decoration_v1;

mod listener;

mod data;
mod deref;
mod drop;
mod get;
mod new;
mod set_decorations;

pub(in crate::window) use listener::XdgTopLevelDecorationListener;

/// A toplevel surface registered with the XDG window manager and with decorations provided by the
/// XDG decoration manager
pub(in crate::window) struct XdgTopLevelDecoration<T> {
    /// The handle to the underlying XDG toplevel decoration
    handle: *mut zxdg_toplevel_decoration_v1,

    /// The XDG toplevel that this decoration is for
    top_level: XdgTopLevel<T>,

    /// A reference to the XDG window manage this toplevel surface came from
    manager: Rc<XdgDecorationManager>,
}
