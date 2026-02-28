use wayland::WaylandWindowSubsystem;

mod wayland;

mod create_window;
mod destroy_window;
mod get_displays;
mod get_windows;
mod new;
mod pump_events;
mod wait_for_event;

pub(in crate::window) use wayland::{
    WaylandBind, WaylandFunctions, WaylandLibrary, WlDisplay, WlRegistryRef, XdgOutputManager,
};

/// The implementation of the [`WindowSubsystem`](crate::window::WindowSubsystem) for Linux
pub(in crate::window) enum WindowSubsystemInner<UserEvent: 'static + Send> {
    /// The Wayland implementation of the window subsystem
    Wayland(WaylandWindowSubsystem<UserEvent>),

    /// The X11 implementation of the window subsystem
    X11,
}
