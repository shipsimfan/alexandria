use crate::{
    Result, WindowError,
    platform::linux::wayland::{XdgSurface, XdgSurfaceListener, XdgWmBase},
};
use std::{
    ptr::{NonNull, null_mut},
    rc::Rc,
};
use wayland::xdg_shell::xdg_surface_add_listener_dyn;

impl XdgSurface {
    /// Add a listener for registry events
    pub fn add_listener<T: XdgSurfaceListener>(mut self, data: T) -> Result<XdgSurface<T>> {
        let mut data = Box::new((data, self.wm.clone()));

        if unsafe {
            xdg_surface_add_listener_dyn(
                self.handle,
                &XdgSurface::<T>::LISTENER,
                (data.as_mut() as *mut (T, Rc<XdgWmBase>)).cast(),
                self.wm.library().f.proxy_add_listener,
            )
        } < 0
        {
            return Err(WindowError::new(
                "unable to set XDG surface listener - listener already set",
            ));
        }

        let handle = self.handle;
        self.handle = null_mut();

        Ok(XdgSurface {
            handle,
            surface: self.surface.take(),
            listener_data: NonNull::new(Box::into_raw(data)),
            wm: self.wm.clone(),
        })
    }
}
