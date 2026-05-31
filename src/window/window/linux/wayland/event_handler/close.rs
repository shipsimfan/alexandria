use crate::{EventKind, Result, window::window::linux::wayland::WaylandEventHandler};

impl<UserEvent: 'static + Send> WaylandEventHandler<UserEvent> {
    /// Send a close request to the window
    pub(in crate::window::window::linux) fn close(&self) -> Result<()> {
        if let Some(id) = self.id {
            self.event_queue
                .push(EventKind::WindowCloseRequest { id })?;
        }

        Ok(())
    }
}
