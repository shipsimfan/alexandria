use crate::{
    EventQueue, PackedMap, Result,
    window::{
        WlCompositor, WlSeat, XdgDecorationManager, XdgOutputManager, XdgWmBase,
        display::DisplayInner, subsystem::linux::wayland::SeatListener,
    },
};
use std::{ffi::CStr, rc::Rc};

mod add_global;
mod enable_events;
mod get;
mod new;
mod registry_listener;
mod remove_global;

/// The bound Wayland globals
pub(in crate::window) struct WaylandGlobals<UserEvent: 'static + Send> {
    /// The queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// Should events be pushed to the event queue?
    events_enabled: bool,

    /// The result of running callbacks
    dispatch_result: Result<()>,

    /// The displays that have been bound from the registry
    displays: PackedMap<DisplayInner<UserEvent>>,

    /// A reference to the global output manager
    xdg_output_manager: Option<Rc<XdgOutputManager>>,

    /// A reference to the global compositor
    compositor: Option<WlCompositor>,

    /// A reference to the XDG window manager
    xdg_wm_base: Option<Rc<XdgWmBase>>,

    /// A reference to the XDG decoration manager
    xdg_decoration_manager: Option<Rc<XdgDecorationManager>>,

    /// The seats that have been bound from the registry
    seats: Vec<WlSeat<SeatListener<UserEvent>>>,

    /// The name of the `wl_output_manager` interface
    wl_output_manager_name: &'static CStr,

    /// The name of the `xdg_output` interface
    xdg_output_name: &'static CStr,

    /// The name of the `xdg_wm_base` interface
    compositor_name: &'static CStr,

    /// The name of the `compositor` interface
    xdg_wm_base_name: &'static CStr,

    /// The name of the `xdg_decoration_manager` interface
    xdg_decoration_manager_name: &'static CStr,

    /// The name of the `wl_seat` interface
    wl_seat_name: &'static CStr,
}
