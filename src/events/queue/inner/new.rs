use crate::{Notify, Result, events::queue::EventQueueInner};
use std::{collections::VecDeque, sync::Mutex};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Create a new [`EventQueueInner`]
    pub(in crate::events::queue) fn new() -> Result<EventQueueInner<UserEvent>> {
        Ok(EventQueueInner {
            queue: Mutex::new(VecDeque::new()),
            notify: Notify::new(false, false)?,
        })
    }
}
