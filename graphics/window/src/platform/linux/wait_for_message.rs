use crate::{Result, Window};

impl Window {
    /// Block for a message from the window system
    ///
    /// On Windows, this function processes the message that caused the thread to wake
    pub fn wait_for_message(&mut self) -> Result<()> {
        todo!()
    }
}
