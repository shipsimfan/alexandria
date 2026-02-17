use crate::{EventPump, Result, events::pump_quit_event};

impl<UserEvent: Send> EventPump<UserEvent> {
    /// Pump events from input devices and the window system onto the event queue
    pub fn pump(&mut self) -> Result<()> {
        pump_quit_event(self)?;

        if let Some(window) = self.context.window_opt() {
            window.pump_events()?;
        }

        Ok(())
    }
}
