use crate::{Result, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Process all messages that have occurred since the last call, or block until one arrives
    pub fn process_messages(&mut self) -> Result<()> {
        self.display.dispatch()
    }
}
