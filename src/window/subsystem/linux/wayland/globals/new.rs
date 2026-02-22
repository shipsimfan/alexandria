use crate::window::subsystem::linux::wayland::WaylandGlobals;

impl WaylandGlobals {
    /// Create a new set of [`WaylandGlobals`]
    pub fn new() -> WaylandGlobals {
        WaylandGlobals {
            dispatch_result: Ok(()),
            /*
            compositor: None,
            xdg_wm_base: None,

            compositor_name: unsafe { CStr::from_ptr(wl_compositor_interface.name) },
            xdg_wm_base_name: unsafe { CStr::from_ptr(xdg_wm_base_interface.name) },
            */
        }
    }
}
