use crate::{EventQueue, Id, math::Recti, window::Window};

mod new;
mod window_proc;

/// The window procedure for normal windows
pub(in crate::window) struct StandardWndProc<UserEvent: 'static + Send> {
    /// The position and size of the window's client area, in screen coordinates
    pub(in crate::window::window::windows) rect: Recti,

    /// The id of the window to push events with
    pub(in crate::window::window::windows) id: Option<Id<Window<UserEvent>>>,

    /// The event queue to which events should be sent
    event_queue: EventQueue<UserEvent>,
}
