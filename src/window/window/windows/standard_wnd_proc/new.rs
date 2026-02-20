use crate::{EventQueue, math::Recti, window::StandardWndProc};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Create a new [`StandardWndProc`]
    pub(in crate::window::window::windows) const fn new(
        event_queue: EventQueue<UserEvent>,
    ) -> StandardWndProc<UserEvent> {
        StandardWndProc {
            rect: Recti::default(),
            id: None,
            event_queue,
        }
    }
}
