use crate::{EventQueue, PackedMap, window::window::WindowInner};
use std::rc::Rc;
use wl_registry::{WlRegistry, WlRegistryListener};

mod globals;
mod library;
mod wl_compositor;
mod wl_display;
mod wl_registry;
mod wl_surface;
mod xdg_output_manager;
mod xdg_surface;
mod xdg_top_level;
mod xdg_wm_base;

mod create_window;
mod destroy_window;
mod get;
mod new;
mod pump_events;
mod wait_for_event;

pub(in crate::window) use globals::WaylandGlobals;
pub(in crate::window) use library::{WaylandFunctions, WaylandLibrary};
pub(in crate::window) use wl_compositor::WlCompositor;
pub(in crate::window) use wl_display::WlDisplay;
pub(in crate::window) use wl_registry::{WaylandBind, WlRegistryRef};
pub(in crate::window) use wl_surface::WlSurface;
pub(in crate::window) use xdg_output_manager::XdgOutputManager;
pub(in crate::window) use xdg_surface::{XdgSurface, XdgSurfaceListener, XdgSurfaceRef};
pub(in crate::window) use xdg_top_level::{XdgTopLevel, XdgTopLevelListener};
pub(in crate::window) use xdg_wm_base::XdgWmBase;

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
