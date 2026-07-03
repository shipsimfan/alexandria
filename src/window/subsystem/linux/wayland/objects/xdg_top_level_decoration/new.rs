use crate::{
    Error, Result,
    window::{
        XdgDecorationManager, XdgTopLevel, XdgTopLevelDecoration, XdgTopLevelDecorationListener,
    },
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_decoration::{
    zxdg_toplevel_decoration_v1, zxdg_toplevel_decoration_v1_add_listener_dyn,
};

impl<T: XdgTopLevelDecorationListener> XdgTopLevelDecoration<T> {
    /// Create a new [`XdgTopLevelDecoration`]
    pub(in crate::window::subsystem::linux::wayland) fn new(
        handle: *mut zxdg_toplevel_decoration_v1,
        top_level: XdgTopLevel<T>,
        manager: Rc<XdgDecorationManager>,
    ) -> Result<XdgTopLevelDecoration<T>> {
        debug_assert_ne!(handle, null_mut());

        let toplevel_decoration = XdgTopLevelDecoration {
            handle,
            top_level,
            manager,
        };

        if unsafe {
            zxdg_toplevel_decoration_v1_add_listener_dyn(
                handle,
                &XdgTopLevelDecoration::<T>::LISTENER,
                toplevel_decoration.top_level.surface().raw_data().cast(),
                *toplevel_decoration.manager.library().f.proxy_add_listener,
            )
        } < 0
        {
            return Err(Error::new(
                "unable to set XDG toplevel listener - listener already set",
            ));
        }

        Ok(toplevel_decoration)
    }
}
