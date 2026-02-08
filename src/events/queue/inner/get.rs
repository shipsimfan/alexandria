use crate::{Notify, events::queue::EventQueueInner};

impl<UserEvent: Send> EventQueueInner<UserEvent> {
    /// Get the [`Notify`] to wait on events being pushed
    pub(in crate::events) fn notify(&self) -> &Notify {
        &self.notify
    }
}
