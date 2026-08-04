use event_handler::WaylandEventHandler;
use handle::WindowHandle;
use crate::math::Vector2u;

mod event_handler;
mod handle;

mod get;
mod new;
mod set;
mod set_id;

/// The Wayland-specific implementation of [`Window`](crate::window::Window)s
pub(in crate::window) struct WaylandWindow<UserEvent: 'static + Send> {
    /// The raw window
    window: WindowHandle<UserEvent>,

    /// The current title of the window
    title: String,

    /// The minimum size of the client area of the window, in pixels
    minimum_size: Option<Vector2u>,

    /// The maximum size of the client area of the window, in pixels
    maximum_size: Option<Vector2u>,
}
