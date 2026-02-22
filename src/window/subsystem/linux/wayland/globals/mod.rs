use crate::Result;

mod get;
mod new;
mod registry_listener;

/// The bound Wayland globals
pub(in crate::window::subsystem::linux::wayland) struct WaylandGlobals {
    /// The result of running callbacks
    dispatch_result: Result<()>,
    /*
    /// A reference to the global compositor
    compositor: Option<WlCompositor>,

    /// A reference to the XDG window manager
    xdg_wm_base: Option<Rc<XdgWmBase>>,

    /// The name of the `xdg_wm_base` interface
    compositor_name: &'static CStr,

    /// The name of the `compositor` interface
    xdg_wm_base_name: &'static CStr,
    */
}
