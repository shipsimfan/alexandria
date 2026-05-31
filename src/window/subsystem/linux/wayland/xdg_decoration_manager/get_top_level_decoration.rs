use crate::{
    Error, Result,
    window::{
        XdgDecorationManager, XdgTopLevel, XdgTopLevelDecoration, XdgTopLevelDecorationListener,
    },
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_decoration::zxdg_decoration_manager_v1_get_toplevel_decoration_dyn;

impl XdgDecorationManager {
    /// Assign the toplevel role to this surface
    pub fn get_top_level_decoration<T: XdgTopLevelDecorationListener>(
        self: &Rc<Self>,
        top_level: XdgTopLevel<T>,
    ) -> Result<XdgTopLevelDecoration<T>> {
        let handle = unsafe {
            zxdg_decoration_manager_v1_get_toplevel_decoration_dyn(
                self.handle,
                top_level.handle(),
                *self.library().f.proxy_marshal_flags,
                *self.library().f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(Error::new("unable to assign XDG toplevel role"));
        }

        XdgTopLevelDecoration::new(handle, top_level, self.clone())
    }
}
