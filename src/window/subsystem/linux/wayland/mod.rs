use globals::WaylandGlobals;
use std::rc::Rc;
use wl_registry::{WlRegistry, WlRegistryListener};

mod globals;
mod library;
mod wl_display;
mod wl_registry;
mod xdg_output_manager;

mod create_window;
mod destroy_window;
mod get;
mod new;
mod pump_events;
mod wait_for_event;

pub(in crate::window) use library::{WaylandFunctions, WaylandLibrary};
pub(in crate::window) use wl_display::WlDisplay;
pub(in crate::window) use wl_registry::{WaylandBind, WlRegistryRef};
pub(in crate::window) use xdg_output_manager::XdgOutputManager;

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Wayland on Linux
pub(in crate::window) struct WaylandWindowSubsystem<UserEvent: 'static + Send> {
    /// The global Wayland registry
    #[allow(unused)]
    registry: WlRegistry<WaylandGlobals<UserEvent>>,

    /// The connection to the Wayland display server
    connection: Rc<WlDisplay>,
}
