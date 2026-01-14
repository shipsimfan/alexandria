use crate::platform::linux::wayland::{XdgToplevel, XdgToplevelListener, XdgWmBase};
use std::{ffi::c_void, rc::Rc};
use wayland::{
    wl_array,
    xdg_shell::{xdg_toplevel, xdg_toplevel_listener},
};

impl<T: XdgToplevelListener> XdgToplevel<T> {
    /// The listeners for the toplevel surface
    pub(in crate::platform::linux::wayland::objects::xdg_toplevel) const LISTENER:
        xdg_toplevel_listener = xdg_toplevel_listener {
        configure: configure_trampoline::<T>,
        close: close_trampoline::<T>,
        configure_bounds: configure_bounds_trampoline::<T>,
        wm_capabilities: wm_capabilities_trampoline,
    };
}

/// Trampoline for responding to a toplevel surface configure event
unsafe extern "C" fn configure_trampoline<T: XdgToplevelListener>(
    data: *mut c_void,
    _: *mut xdg_toplevel,
    width: i32,
    height: i32,
    _: *mut wl_array,
) {
    let data: &mut (T, Rc<XdgWmBase>) = unsafe { &mut *data.cast() };

    data.0.configure(width, height);
}

/// Trampoline for responding to a close request
unsafe extern "C" fn close_trampoline<T: XdgToplevelListener>(
    data: *mut c_void,
    _: *mut xdg_toplevel,
) {
    let data: &mut (T, Rc<XdgWmBase>) = unsafe { &mut *data.cast() };

    data.0.close();
}

/// Trampoline for responding to a toplevel surface configure bounds event
unsafe extern "C" fn configure_bounds_trampoline<T: XdgToplevelListener>(
    data: *mut c_void,
    _: *mut xdg_toplevel,
    width: i32,
    height: i32,
) {
    let data: &mut (T, Rc<XdgWmBase>) = unsafe { &mut *data.cast() };

    data.0.configure_bounds(width, height);
}

/// Trampoline for responding to a close request
unsafe extern "C" fn wm_capabilities_trampoline(
    _: *mut c_void,
    _: *mut xdg_toplevel,
    _: *mut wl_array,
) {
}
