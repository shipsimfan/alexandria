use crate::{
    Error, Result,
    window::{XdgSurface, XdgTopLevel, XdgTopLevelListener},
};
use std::ptr::null_mut;
use wayland::xdg_shell::xdg_surface_get_toplevel_dyn;

impl<T: XdgTopLevelListener> XdgSurface<T> {
    /// Assign the toplevel role to this surface
    pub fn get_top_level(self) -> Result<XdgTopLevel<T>> {
        let handle = unsafe {
            xdg_surface_get_toplevel_dyn(
                self.handle,
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(Error::new("unable to assign XDG toplevel role"));
        }

        let wm = self.wm.clone();
        XdgTopLevel::new(handle, self, wm)
    }
}
