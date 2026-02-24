use crate::{EventQueue, math::Recti, window::StandardWndProc};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Create a new [`StandardWndProc`]
    pub(in crate::window::window::windows) const fn new(
        event_queue: EventQueue<UserEvent>,
    ) -> StandardWndProc<UserEvent> {
        StandardWndProc {
            rect: Recti::default(),
            windowed_rect: Recti::default(),
            is_fullscreen: false,

            id: None,
            event_queue,
        }
    }
}
