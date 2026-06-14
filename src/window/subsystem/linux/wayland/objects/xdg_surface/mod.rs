use crate::window::{WlSurface, XdgWmBase};
use std::{ptr::NonNull, rc::Rc};
use wayland::xdg_shell::xdg_surface;

mod listener;
mod r#ref;

mod data;
mod deref;
mod drop;
mod get;
mod get_top_level;
mod new;

pub(in crate::window) use listener::XdgSurfaceListener;
pub(in crate::window) use r#ref::XdgSurfaceRef;

/// A surface register with the XDG window manager
pub(in crate::window) struct XdgSurface<T = ()> {
    /// The handle to the underlying XDG surface
    handle: *mut xdg_surface,

    /// The Wayland surface that this XDG surface is for
    surface: Option<WlSurface>,

    /// The data used for listener callbacks
    listener_data: Option<NonNull<(T, Rc<XdgWmBase>)>>,

    /// A reference to the XDG window manage this surface came from
    wm: Rc<XdgWmBase>,
}
