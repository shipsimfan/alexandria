use crate::{EventQueue, Id, PackedMap, Result, window::display::DisplayInner};
use std::ffi::CStr;

mod add_global;
mod enable_events;
mod get;
mod new;
mod registry_listener;
mod remove_global;

/// The bound Wayland globals
pub(in crate::window::subsystem::linux::wayland) struct WaylandGlobals<UserEvent: 'static + Send> {
    /// The queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// Should events be pushed to the event queue?
    events_enabled: bool,

    /// The result of running callbacks
    dispatch_result: Result<()>,

    /// The displays that have been bound from the registry
    displays: PackedMap<DisplayInner<UserEvent>>,

    /// A mapping from the global name to the display it is associated with
    name_to_display_map: Vec<(u32, Id<DisplayInner<UserEvent>>)>,
    /*
    /// A reference to the global compositor
    compositor: Option<WlCompositor>,

    /// A reference to the XDG window manager
    xdg_wm_base: Option<Rc<XdgWmBase>>,
    */
    wl_output_name: &'static CStr,
    /*
    /// The name of the `xdg_wm_base` interface
    compositor_name: &'static CStr,

    /// The name of the `compositor` interface
    xdg_wm_base_name: &'static CStr,
    */
}
