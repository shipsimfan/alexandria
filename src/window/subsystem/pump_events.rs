use crate::{Result, window::WindowSubsystem};

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Pumps events from input devices and the window system onto the event queue
    pub(crate) fn pump_events(&self) -> Result<()> {
        self.inner.borrow_mut().pump_events()
    }
}
