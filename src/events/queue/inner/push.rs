use crate::{Event, EventKind, Result, events::queue::EventQueueInner};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Insert a new event into the queue
    pub fn push<K: Into<EventKind<UserEvent>>>(&mut self, event: K) -> Result<()> {
        self.queue.lock().unwrap().push_back(Event::new(event));
        self.notify.notify()
    }
}
