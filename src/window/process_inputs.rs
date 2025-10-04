use crate::Window;
use std::ptr::null_mut;
use win32::{DispatchMessage, PeekMessage, TranslateMessage, MSG, PM_REMOVE};

impl Window {
    /// Process all inputs and events that have occurred since the last call
    pub fn process_inputs(&mut self) {
        let mut msg = MSG::default();
        while unsafe { PeekMessage(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessage(&msg) };
        }
    }
}
