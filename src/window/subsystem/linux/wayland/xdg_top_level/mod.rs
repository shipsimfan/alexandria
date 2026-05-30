use crate::window::{XdgSurface, XdgWmBase};
use std::rc::Rc;
use wayland::xdg_shell::xdg_toplevel;

mod listener;

mod commit;
mod data;
mod deref;
mod drop;
mod get;
mod new;
mod set_title;

pub(in crate::window) use listener::XdgTopLevelListener;

/// A toplevel surface registered with the XDG window manager
pub(in crate::window) struct XdgTopLevel<T> {
    /// The handle to the underlying XDG surface
    handle: *mut xdg_toplevel,

    /// The XDG surface that this toplevel role is for
    surface: XdgSurface<T>,

    /// A reference to the XDG window manage this toplevel surface came from
    wm: Rc<XdgWmBase>,
}
