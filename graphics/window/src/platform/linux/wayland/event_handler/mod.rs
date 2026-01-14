use crate::{WindowEvents, WindowState};

mod get;
mod new;
mod surface_listener;
mod toplevel_listener;

pub(in crate::platform::linux::wayland) struct WaylandEventHandler<Callbacks: WindowEvents> {
    /// The current state of the window
    state: WindowState,

    /// Did a resize occur this pump?
    did_resize: bool,

    /// Did a maximize or restore occur this pump?
    did_maximize_or_restore: bool,

    /// Did a close request occur this pump?
    did_close_request: bool,

    /// The user provided callbacks for window events
    callbacks: Callbacks,
}
