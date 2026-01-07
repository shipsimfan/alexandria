use crate::platform::linux::wayland::WlDisplay;
use std::rc::Rc;
use wayland::xdg_shell::xdg_wm_base;

mod bind;
mod drop;
mod register_ping_handler;

/// A reference to the XDG window manager
pub(in crate::platform::linux::wayland) struct XdgWmBase {
    /// The handle to the XDG window manager
    handle: *mut xdg_wm_base,

    /// The display that this window manager comes from
    display: Rc<WlDisplay>,
}
