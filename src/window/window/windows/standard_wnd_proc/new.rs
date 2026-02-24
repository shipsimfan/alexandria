use crate::{
    EventQueue,
    math::Recti,
    window::{StandardWndProc, WindowStyle},
};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Create a new [`StandardWndProc`]
    pub(in crate::window::window::windows) const fn new(
        style: WindowStyle,
        event_queue: EventQueue<UserEvent>,
    ) -> StandardWndProc<UserEvent> {
        StandardWndProc {
            style,

            rect: Recti::default(),
            windowed_rect: Recti::default(),

            minimum_client_size: None,
            minimum_window_size: None,
            maximum_client_size: None,
            maximum_window_size: None,

            is_fullscreen: false,

            id: None,
            event_queue,
        }
    }
}
