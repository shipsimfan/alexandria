use crate::{EventQueue, Result, events::queue::EventQueueInner};

impl<UserEvent: Send> EventQueue<UserEvent> {
    /// Create a new [`EventQueue`]
    pub(crate) fn new() -> Result<EventQueue<UserEvent>> {
        EventQueueInner::new().map(EventQueue::from_inner)
    }
}
