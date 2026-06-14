use crate::{EventQueue, PackedMap, window::window::WindowInner};
use objects::{WlRegistry, WlRegistryListener};
use seat_listener::SeatListener;
use std::rc::Rc;

mod globals;
mod library;
mod objects;
mod seat_listener;

mod create_window;
mod destroy_window;
mod get;
mod new;
mod pump_events;
mod wait_for_event;

pub(in crate::window) use globals::WaylandGlobals;
pub(in crate::window) use library::{WaylandFunctions, WaylandLibrary};
pub(in crate::window) use objects::{
    WaylandBind, WlCompositor, WlDisplay, WlKeyboard, WlKeyboardListener, WlRegistryRef, WlSeat,
    WlSeatListener, WlSeatRef, WlSurface, XdgDecorationManager, XdgOutputManager, XdgSurface,
    XdgSurfaceListener, XdgSurfaceRef, XdgTopLevel, XdgTopLevelDecoration,
    XdgTopLevelDecorationListener, XdgTopLevelListener, XdgWmBase,
};

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Wayland on Linux
pub(in crate::window) struct WaylandWindowSubsystem<UserEvent: 'static + Send> {
    /// The global Wayland registry
    #[allow(unused)]
    registry: WlRegistry<WaylandGlobals<UserEvent>>,

    /// The connection to the Wayland display server
    connection: Rc<WlDisplay>,

    /// The event queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// The windows that have been created
    windows: PackedMap<WindowInner<UserEvent>>,
}
