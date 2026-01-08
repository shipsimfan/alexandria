use crate::platform::linux::wayland::XdgWmBase;
use std::rc::Rc;
use wayland::xdg_shell::xdg_toplevel;

mod new;

/// A temporary reference to an [`XdgToplevel`](crate::platform::linux::wayland::XdgToplevel)
pub(in crate::platform::linux::wayland) struct XdgToplevelRef<'a> {
    /// The raw handle to the toplevel surface
    handle: *mut xdg_toplevel,

    /// The window manager this toplevel surface comes from
    wm: &'a Rc<XdgWmBase>,
}
