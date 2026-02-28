use event_handler::WaylandDisplayEventHandler;
use wl_output::WlOutputListener;

mod event_handler;
mod wl_output;
mod xdg_output;

mod downgrade;
mod get;
mod new;
mod set_display_id;
mod upgrade;

pub(in crate::window) use wl_output::WlOutput;
pub(in crate::window) use xdg_output::{XdgOutput, XdgOutputListener};

/// The implementation of [`Display`](crate::window::Display)s for Wayland on Linux
#[allow(private_interfaces)]
pub(in crate::window) enum WaylandDisplay<UserEvent: 'static + Send> {
    /// The output is only a [`WlOutput`]
    Wl(WlOutput<WaylandDisplayEventHandler<UserEvent>>),

    /// The output is both a [`WlOutput`] and an [`XdgOutput`]
    Xdg(XdgOutput<WaylandDisplayEventHandler<UserEvent>>),
}
