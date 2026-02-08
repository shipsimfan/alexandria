use crate::{Event, EventPump, Result};
use std::time::Duration;

impl<UserEvent: Send> EventPump<UserEvent> {
    /// Wait indefinetly until an event is available
    pub fn wait(&mut self) -> Result<Event<UserEvent>> {
        self.wait_timeout(None).map(|event| event.unwrap())
    }

    /// Wait until an event is available or `timeout` passes
    pub fn wait_timeout(&mut self, timeout: Option<Duration>) -> Result<Option<Event<UserEvent>>> {
        loop {
            if let Some(event) = self.poll()? {
                return Ok(Some(event));
            }

            let signalled = if let Some(window) = self.context.window_opt() {
                window.wait_for_event(self.notify(), timeout)?
            } else {
                self.notify().wait(timeout)?
            };

            if !signalled {
                return Ok(None);
            }
        }
    }
}
