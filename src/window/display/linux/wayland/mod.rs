use event_handler::WaylandDisplayEventHandler;
use wl_output::{WlOutput, WlOutputListener};

mod event_handler;
mod set_display_id;
mod wl_output;

mod get;
mod new;

/// The implementation of [`Display`](crate::window::Display)s for Wayland on Linux
pub(in crate::window) struct WaylandDisplay<UserEvent: 'static + Send> {
    /// The wayland output this display is associated with
    output: WlOutput<WaylandDisplayEventHandler<UserEvent>>,
}
