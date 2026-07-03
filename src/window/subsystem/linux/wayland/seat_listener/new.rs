use crate::{
    EventQueue, Result,
    window::subsystem::linux::wayland::{SeatListener, seat_listener::XkbContext},
};

impl<UserEvent: 'static + Send> SeatListener<UserEvent> {
    /// Create a new [`SeatListener`]
    pub fn new(event_queue: EventQueue<UserEvent>) -> Result<SeatListener<UserEvent>> {
        Ok(SeatListener {
            event_queue,
            xkb_context: XkbContext::new()?,
            keyboard: None,
        })
    }
}
