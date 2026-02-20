use crate::{EventQueue, Notify};

impl<UserEvent: 'static + Send> EventQueue<UserEvent> {
    /// Get the [`Notify`] to wait on events being pushed
    pub(in crate::events) fn notify(&self) -> &Notify {
        self.inner.notify()
    }
}
