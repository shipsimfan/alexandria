use crate::{Result, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Process all messages that have occurred since the last call
    ///
    /// If none have happened, this function will return immediately
    pub fn process_messages(&mut self) -> Result<()> {
        Ok(())
    }
}
