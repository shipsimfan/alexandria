use crate::{
    Result, WindowError,
    platform::linux::wayland::{XdgSurface, XdgToplevel, XdgToplevelListener},
};
use std::ptr::null_mut;
use wayland::xdg_shell::xdg_surface_get_toplevel_dyn;

impl<T: XdgToplevelListener> XdgSurface<T> {
    /// Assign the toplevel role to this surface
    pub fn get_toplevel(self) -> Result<XdgToplevel<T>> {
        let handle = unsafe {
            xdg_surface_get_toplevel_dyn(
                self.handle,
                self.wm.library().f.proxy_marshal_flags,
                self.wm.library().f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(WindowError::new("unable to assign XDG toplevel role"));
        }

        let wm = self.wm.clone();
        XdgToplevel::new(handle, self, wm)
    }
}
