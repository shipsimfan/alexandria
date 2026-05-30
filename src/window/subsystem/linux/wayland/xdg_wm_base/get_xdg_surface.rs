use crate::{
    Error, Result,
    window::{WlSurface, XdgSurface, XdgWmBase},
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::xdg_wm_base_get_xdg_surface_dyn;

impl XdgWmBase {
    /// Get an [`XdgSurface`] for `surface`
    pub fn get_xdg_surface(self: Rc<Self>, surface: WlSurface) -> Result<XdgSurface> {
        let handle = unsafe {
            xdg_wm_base_get_xdg_surface_dyn(
                self.handle,
                surface.handle(),
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(Error::new("unable to create an XDG surface"));
        }

        Ok(XdgSurface::new(handle, surface, self))
    }
}
