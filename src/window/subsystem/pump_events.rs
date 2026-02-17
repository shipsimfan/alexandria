use crate::{EventQueue, Result, window::WindowSubsystem};

impl WindowSubsystem {
    /// Pumps events from input devices and the window system onto the event queue
    pub(crate) fn pump_events<UserEvent: Send>(&self, pump: &EventQueue<UserEvent>) -> Result<()> {
        self.inner.borrow_mut().pump_events(pump)
    }
}
