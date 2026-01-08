use crate::platform::linux::wayland::{XdgSurface, XdgWmBase};
use std::rc::Rc;
use wayland::xdg_shell::xdg_toplevel;

mod listener;
mod r#ref;

mod data;
mod drop;
mod get;
mod new;

pub(in crate::platform::linux::wayland) use listener::XdgToplevelListener;
pub(in crate::platform::linux::wayland) use r#ref::XdgToplevelRef;

/// A toplevel surface registered with the XDG window manager
pub struct XdgToplevel<T> {
    /// The handle to the underlying XDG surface
    handle: *mut xdg_toplevel,

    /// The XDG surface that this toplevel role is for
    surface: XdgSurface<T>,

    /// A reference to the XDG window manage this toplevel surface came from
    wm: Rc<XdgWmBase>,
}
