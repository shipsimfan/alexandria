use crate::{
    EventQueue, Id,
    math::{Recti, Vector2u},
    window::{Window, WindowStyle},
};

mod get;
mod init;
mod new;
mod set_id;
mod set_maximum_size;
mod set_minimum_size;
mod window_proc;

/// The window procedure for normal windows
pub(in crate::window) struct StandardWndProc<UserEvent: 'static + Send> {
    /// The currently applied window style
    style: WindowStyle,

    /// The current position and size of the window's client area, in screen coordinates
    rect: Recti,

    /// The position and size of the window when it is windowed, in screen coordinates
    ///
    /// If the window is not currently fullscreen, this will be the same as [`rect`](Self::rect)
    ///
    /// If the window is currently fullscreen, this will be the position and size of the window
    /// when it is restored to windowed mode
    windowed_rect: Recti,

    /// The requested minimum size of the window's client area, in pixels
    minimum_client_size: Option<Vector2u>,

    /// The requested minimum size of the window, in pixels
    minimum_window_size: Option<Vector2u>,

    /// The requested maximum size of the window's client area, in pixels
    maximum_client_size: Option<Vector2u>,

    /// The requested maximum size of the window, in pixels
    maximum_window_size: Option<Vector2u>,

    /// Is this window currently fullscreen?
    is_fullscreen: bool,

    /// The id of the window to push events with
    id: Option<Id<Window<UserEvent>>>,

    /// The event queue to which events should be sent
    event_queue: EventQueue<UserEvent>,
}
