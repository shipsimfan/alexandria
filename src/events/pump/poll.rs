use crate::{Event, EventPump, Result};

impl<UserEvent: 'static + Send> EventPump<UserEvent> {
    /// Poll for an event
    pub fn poll(&mut self) -> Result<Option<Event<UserEvent>>> {
        self.pump()?;
        self.pop()
    }
}
