use crate::{
    EventQueue, Id,
    math::{Recti, Vector2u},
    window::Window,
};

mod close;
mod get;
mod new;
mod set_id;
mod surface_listeners;
mod top_level_decoration_listeners;
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

    /// The position and size of the window when it is windowed, in screen coordinates
    ///
    /// If the window is not currently fullscreen, this will be the same as [`rect`](Self::rect)
    ///
    /// If the window is currently fullscreen, this will be the position and size of the window
    /// when it is restored to windowed mode
    windowed_rect: Recti,

    /// The size requested by Wayland for the window's client area, in screen coordinates
    requested_size: Vector2u,

    /// Is this window currently fullscreen?
    is_fullscreen: bool,

    /// Is this window currently maximized?
    is_maximized: bool,

    /// Is this window currently focused?
    is_focused: bool,

    /// Is the window borderless?
    is_borderless: bool,
}
