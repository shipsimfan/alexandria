use crate::{EventKind, Result, window::StandardWndProc};

impl<UserEvent: 'static + Send> StandardWndProc<UserEvent> {
    /// Send a close request message for a window
    pub(in crate::window::window::windows) fn close(&self) -> Result<()> {
        self.event_queue.push(EventKind::WindowCloseRequest {
            id: self.id.unwrap(),
        })
    }
}
