use crate::window::XdgTopLevel;
use event_handler::WaylandEventHandler;

mod event_handler;

mod get;
mod new;
mod set_id;

/// The Wayland-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) struct WaylandWindow<UserEvent: 'static + Send> {
    /// The raw window
    window: XdgTopLevel<WaylandEventHandler<UserEvent>>,

    /// The current title of the window
    title: String,
}
