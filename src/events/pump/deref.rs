use crate::{EventPump, EventQueue};
use std::ops::Deref;

impl<UserEvent: 'static + Send> Deref for EventPump<UserEvent> {
    type Target = EventQueue<UserEvent>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}
