use crate::{Result, Window};
use std::ptr::null_mut;
use win32::{DispatchMessage, MSG, PM_REMOVE, PeekMessage, TranslateMessage, WM_QUIT};

impl Window {
    /// Process all messages that have occurred since the last call
    ///
    /// If none have happened, this function will return immediately
    pub fn process_messages(&mut self, max_messages: Option<u32>) -> Result<()> {
        let mut msg = MSG::default();
        let mut messages_processed = 0;
        while max_messages
            .map(|max| messages_processed < max)
            .unwrap_or(true)
            && self.is_running()
            && unsafe { PeekMessage(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0
        {
            self.process_message(&msg)?;
            messages_processed += 1;
        }

        Ok(())
    }

    /// Process a single message from Windows
    pub(in crate::platform::windows) fn process_message(&mut self, msg: &MSG) -> Result<()> {
        if msg.message == WM_QUIT {
            self.state.set_is_running(false);
        }

        unsafe { TranslateMessage(msg) };
        unsafe { DispatchMessage(msg) };

        if self.wnd_proc_result.is_err() {
            let mut result = Ok(());
            std::mem::swap(&mut result, &mut self.wnd_proc_result);
            return result;
        }

        Ok(())
    }
}
