use crate::platform::linux::wayland::{XdgWmBase, objects::XdgToplevelRef};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_toplevel;

impl<'a> XdgToplevelRef<'a> {
    /// Create a new [`XdgToplevelRef`]
    pub(in crate::platform::linux::wayland::objects::xdg_toplevel) fn new(
        handle: *mut xdg_toplevel,
        wm: &'a Rc<XdgWmBase>,
    ) -> XdgToplevelRef<'a> {
        assert_ne!(handle, null_mut());

        XdgToplevelRef { handle, wm }
    }
}
