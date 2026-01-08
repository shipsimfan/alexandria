use crate::platform::linux::wayland::{XdgSurface, XdgSurfaceListener, XdgSurfaceRef, XdgWmBase};
use std::{ffi::c_void, rc::Rc};
use wayland::xdg_shell::{xdg_surface, xdg_surface_listener};

impl<T: XdgSurfaceListener> XdgSurface<T> {
    /// The listeners for the surface
    pub(in crate::platform::linux::wayland::objects::xdg_surface::listener) const LISTENER:
        xdg_surface_listener = xdg_surface_listener {
        configure: configure_trampoline::<T>,
    };
}

/// Trampoline for responding to a surface configure event
unsafe extern "C" fn configure_trampoline<T: XdgSurfaceListener>(
    data: *mut c_void,
    surface: *mut xdg_surface,
    serial: u32,
) {
    let data: &mut (T, Rc<XdgWmBase>) = unsafe { &mut *data.cast() };

    let surface = XdgSurfaceRef::new(surface, &data.1);

    data.0.configure(surface, serial);
}
