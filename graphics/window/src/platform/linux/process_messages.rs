use crate::{Result, Window};

impl Window {
    /// Process all messages that have occurred since the last call
    ///
    /// If none have happened, this function will return immediately
    pub fn process_messages(&mut self, max_messages: Option<u32>) -> Result<()> {
        todo!()
    }
}
