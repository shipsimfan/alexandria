use crate::{
    Result, WindowError,
    platform::linux::wayland::{XdgSurface, XdgToplevel, XdgToplevelListener, XdgWmBase},
};
use std::{ptr::null_mut, rc::Rc};
use wayland::xdg_shell::{xdg_toplevel, xdg_toplevel_add_listener_dyn};

impl<T: XdgToplevelListener> XdgToplevel<T> {
    /// Create a new [`XdgToplevel`]
    pub(in crate::platform::linux::wayland::objects) fn new(
        handle: *mut xdg_toplevel,
        surface: XdgSurface<T>,
        wm: Rc<XdgWmBase>,
    ) -> Result<XdgToplevel<T>> {
        debug_assert_ne!(handle, null_mut());

        let toplevel = XdgToplevel {
            handle,
            surface,
            wm,
        };

        if unsafe {
            xdg_toplevel_add_listener_dyn(
                handle,
                &XdgToplevel::<T>::LISTENER,
                toplevel.surface.raw_data().cast(),
                toplevel.wm.library().f.proxy_add_listener,
            )
        } < 0
        {
            return Err(WindowError::new(
                "unable to set XDG toplevel listener - listener already set",
            ));
        }

        Ok(toplevel)
    }
}
