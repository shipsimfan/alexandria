use crate::window::{XdgTopLevelDecoration, XdgTopLevelDecorationListener, XdgWmBase};
use std::{ffi::c_void, rc::Rc};
use wayland::xdg_decoration::{
    zxdg_toplevel_decoration_v1, zxdg_toplevel_decoration_v1_listener,
    zxdg_toplevel_decoration_v1_mode,
};

impl<T: XdgTopLevelDecorationListener> XdgTopLevelDecoration<T> {
    /// The listeners for the toplevel surface
    pub(in crate::window::subsystem::linux::wayland::xdg_top_level_decoration) const LISTENER:
        zxdg_toplevel_decoration_v1_listener = zxdg_toplevel_decoration_v1_listener {
        configure: configure_trampoline::<T>,
    };
}

/// Trampoline for responding to a toplevel surface configure event
unsafe extern "C" fn configure_trampoline<T: XdgTopLevelDecorationListener>(
    data: *mut c_void,
    _: *mut zxdg_toplevel_decoration_v1,
    mode: u32,
) {
    let data: &mut (T, Rc<XdgWmBase>) = unsafe { &mut *data.cast() };

    data.0
        .configure(mode == zxdg_toplevel_decoration_v1_mode::ServerSide as _);
}
