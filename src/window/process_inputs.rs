use crate::{Result, Window};
use std::ptr::null_mut;
use win32::{DispatchMessage, PeekMessage, TranslateMessage, MSG, PM_REMOVE};

impl<LogCallbacks: crate::LogCallbacks, Input: crate::input::Input> Window<LogCallbacks, Input> {
    /// Process all inputs and events that have occurred since the last call
    pub fn process_inputs(&mut self) -> Result<()> {
        let mut msg = MSG::default();
        while unsafe { PeekMessage(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessage(&msg) };

            if self.wnd_proc_result.is_err() {
                let mut result = Ok(());
                std::mem::swap(&mut result, &mut self.wnd_proc_result);
                return result;
            }
        }

        Ok(())
    }
}
