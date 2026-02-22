use crate::EventQueue;
use globals::WaylandGlobals;
use library::{WaylandFunctions, WaylandLibrary};
use std::rc::Rc;
use wl_registry::{WaylandBind, WlRegistry, WlRegistryListener, WlRegistryRef};

mod globals;
mod library;
mod wl_display;
mod wl_registry;

mod new;
mod pump_events;
mod wait_for_event;

pub(in crate::window::subsystem::linux) use wl_display::WlDisplay;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Wayland on Linux
pub(in crate::window::subsystem::linux) struct WaylandWindowSubsystem<UserEvent: 'static + Send> {
    /// The connection to the Wayland display server
    connection: Rc<WlDisplay>,

    /// The event queue to push events to
    event_queue: EventQueue<UserEvent>,
}
