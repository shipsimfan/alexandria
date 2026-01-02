use crate::{Result, Window, WindowError};
use std::ptr::null_mut;
use win32::{GetMessage, MSG};

impl Window {
    /// Block for a message from the window system
    ///
    /// On Windows, this function processes the message that caused the thread to wake
    pub fn wait_for_message(&mut self) -> Result<()> {
        let mut msg = MSG::default();
        if unsafe { GetMessage(&mut msg, null_mut(), 0, 0) } == -1 {
            return Err(WindowError::new_os(
                "unable to wait for message",
                win32::Error::get_last_error(),
            ));
        }
        self.process_message(&msg)
    }
}
