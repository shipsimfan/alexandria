use crate::platform::linux::wayland::WaylandGlobals;
use std::ffi::CStr;
use wayland::wl_compositor_interface;

impl WaylandGlobals {
    /// Create a new set of [`WaylandGlobals`]
    pub fn new() -> WaylandGlobals {
        WaylandGlobals {
            dispatch_result: Ok(()),

            compositor: None,

            compositor_name: unsafe { CStr::from_ptr(wl_compositor_interface.name) },
        }
    }
}
