use crate::{EventPump, Result};

impl<UserEvent: Send> EventPump<UserEvent> {
    /// Pump events from input devices and the window system onto the event queue
    pub fn pump(&mut self) -> Result<()> {
        if let Some(window) = self.context.window_opt() {
            window.pump_events()?;
        }

        Ok(())
    }
}
