use crate::{Notify, events::queue::EventQueueInner};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Get the [`Notify`] to wait on events being pushed
    pub fn notify(&self) -> &Notify {
        &self.notify
    }
}
