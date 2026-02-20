use crate::{EventKind, EventQueue, Result};

impl<UserEvent: 'static + Send> EventQueue<UserEvent> {
    /// Insert a new event into the queue
    pub fn push<K: Into<EventKind<UserEvent>>>(&self, event: K) -> Result<()> {
        self.inner.push(event)
    }
}
