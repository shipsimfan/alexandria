use crate::{Result, window::subsystem::linux::WaylandWindowSubsystem};

impl<UserEvent: 'static + Send> WaylandWindowSubsystem<UserEvent> {
    /// Pumps events from input devices and the window system onto the event queue
    pub(in crate::window::subsystem::linux) fn pump_events(&mut self) -> Result<()> {
        // Dispatch events before reading
        while !self.connection.prepare_read() {
            self.connection.dispatch_pending()?;
        }

        // Flush pending requests
        if let Err(error) = self.connection.flush() {
            self.connection.cancel_read();
            return Err(error);
        }

        // Read new events
        self.connection.read_events()?;

        // Dispatch pending events
        self.connection.dispatch_pending()
    }
}
