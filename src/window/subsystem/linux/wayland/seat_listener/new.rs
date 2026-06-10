use crate::{EventQueue, window::subsystem::linux::wayland::SeatListener};

impl<UserEvent: 'static + Send> SeatListener<UserEvent> {
    /// Create a new [`SeatListener`]
    pub fn new(event_queue: EventQueue<UserEvent>) -> SeatListener<UserEvent> {
        SeatListener {
            event_queue,
            keyboard: None,
            keymap: None,
        }
    }
}
