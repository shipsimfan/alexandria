use crate::{Event, EventQueue, Result};

impl<UserEvent: 'static + Send> EventQueue<UserEvent> {
    /// Take an event from the queue without trying to pump any events
    pub fn pop(&self) -> Result<Option<Event<UserEvent>>> {
        self.inner.pop()
    }
}
