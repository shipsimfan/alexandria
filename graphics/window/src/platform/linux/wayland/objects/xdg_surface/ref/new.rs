use crate::platform::linux::wayland::{XdgWmBase, objects::XdgSurfaceRef};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_surface;

impl<'a> XdgSurfaceRef<'a> {
    pub(in crate::platform::linux::wayland::objects::xdg_surface) fn new(
        handle: *mut xdg_surface,
        wm: &'a Rc<XdgWmBase>,
    ) -> XdgSurfaceRef<'a> {
        assert_ne!(handle, null_mut());

        XdgSurfaceRef { handle, wm }
    }
}
