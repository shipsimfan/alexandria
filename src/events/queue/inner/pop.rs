use crate::{Event, Result, events::queue::EventQueueInner};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Take an event from the queue without trying to pump any events
    pub fn pop(&self) -> Result<Option<Event<UserEvent>>> {
        let mut queue = self.queue.lock().unwrap();
        self.notify.reset()?;
        Ok(queue.pop_front())
    }
}
