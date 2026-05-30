use crate::{EventQueue, Id, math::Recti, window::Window};

mod get;
mod new;
mod set_id;
mod surface_listeners;
mod top_level_listeners;

/// The handler for events from a Wayland window
pub(in crate::window::window::linux::wayland) struct WaylandEventHandler<UserEvent: 'static + Send>
{
    /// The id of the window to push events with
    id: Option<Id<Window<UserEvent>>>,

    /// The event queue to which events should be sent
    event_queue: EventQueue<UserEvent>,

    /// The current position and size of the window's client area, in screen coordinates
    rect: Recti,

    /// Is this window currently fullscreen?
    is_fullscreen: bool,

    /// Is this window currently maximized?
    is_maximized: bool,

    /// Is this window currently minimized?
    is_minimized: bool,
}
