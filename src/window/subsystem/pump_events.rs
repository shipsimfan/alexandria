use crate::{Result, window::WindowSubsystem};

impl WindowSubsystem {
    /// Pumps events from input devices and the window system onto the event queue
    pub(crate) fn pump_events(&self) -> Result<()> {
        Ok(())
    }
}
