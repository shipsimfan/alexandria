use crate::{Result, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Block for a message from the window system
    pub fn wait_for_message(&mut self) -> Result<()> {
        self.display.dispatch()
    }
}
