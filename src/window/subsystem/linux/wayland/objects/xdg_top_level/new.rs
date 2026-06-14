use crate::{
    Error, Result,
    window::{XdgSurface, XdgTopLevel, XdgTopLevelListener, XdgWmBase},
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::{xdg_toplevel, xdg_toplevel_add_listener_dyn};

impl<T: XdgTopLevelListener> XdgTopLevel<T> {
    /// Create a new [`XdgTopLevel`]
    pub(in crate::window::subsystem::linux::wayland) fn new(
        handle: *mut xdg_toplevel,
        surface: XdgSurface<T>,
        wm: Rc<XdgWmBase>,
    ) -> Result<XdgTopLevel<T>> {
        debug_assert_ne!(handle, null_mut());

        let toplevel = XdgTopLevel {
            handle,
            surface,
            wm,
        };

        if unsafe {
            xdg_toplevel_add_listener_dyn(
                handle,
                &XdgTopLevel::<T>::LISTENER,
                toplevel.surface.raw_data().cast(),
                *toplevel.wm.library().f.proxy_add_listener,
            )
        } < 0
        {
            return Err(Error::new(
                "unable to set XDG toplevel listener - listener already set",
            ));
        }

        Ok(toplevel)
    }
}
