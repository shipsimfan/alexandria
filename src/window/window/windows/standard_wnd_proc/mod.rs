use crate::{EventQueue, Id, math::Recti, window::Window};

mod get;
mod init;
mod new;
mod set_id;
mod window_proc;

/// The window procedure for normal windows
pub(in crate::window) struct StandardWndProc<UserEvent: 'static + Send> {
    /// The current position and size of the window's client area, in screen coordinates
    rect: Recti,

    /// The position and size of the window when it is windowed, in screen coordinates
    ///
    /// If the window is not currently fullscreen, this will be the same as [`rect`](Self::rect)
    ///
    /// If the window is currently fullscreen, this will be the position and size of the window
    /// when it is restored to windowed mode
    windowed_rect: Recti,

    /// Is this window currently fullscreen?
    is_fullscreen: bool,

    /// The id of the window to push events with
    id: Option<Id<Window<UserEvent>>>,

    /// The event queue to which events should be sent
    event_queue: EventQueue<UserEvent>,
}
