use crate::{EventQueue, PackedMap, window::WaylandGlobals};
use std::ffi::CStr;
use wayland::{
    wl_compositor_interface, wl_output_interface,
    xdg_decoration::zxdg_decoration_manager_v1_interface,
    xdg_output::zxdg_output_manager_v1_interface, xdg_shell::xdg_wm_base_interface,
};

impl<UserEvent: 'static + Send> WaylandGlobals<UserEvent> {
    /// Create a new set of [`WaylandGlobals`]
    pub fn new(event_queue: EventQueue<UserEvent>) -> WaylandGlobals<UserEvent> {
        WaylandGlobals {
            event_queue,
            events_enabled: false,
            dispatch_result: Ok(()),

            displays: PackedMap::new(),

            xdg_output_manager: None,

            compositor: None,
            xdg_wm_base: None,
            xdg_decoration_manager: None,

            wl_output_manager_name: unsafe { CStr::from_ptr(wl_output_interface.name) },
            xdg_output_name: unsafe { CStr::from_ptr(zxdg_output_manager_v1_interface.name) },

            compositor_name: unsafe { CStr::from_ptr(wl_compositor_interface.name) },
            xdg_wm_base_name: unsafe { CStr::from_ptr(xdg_wm_base_interface.name) },
            xdg_decoration_manager_name: unsafe {
                CStr::from_ptr(zxdg_decoration_manager_v1_interface.name)
            },
        }
    }
}
