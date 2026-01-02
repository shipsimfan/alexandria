use crate::WindowWakeHandleInner;
use std::sync::{Arc, Mutex};
use win32::GetCurrentThreadId;

impl WindowWakeHandleInner {
    /// Create a new [`WindowWakeHandleInner`]
    pub(in crate::platform::windows) fn new() -> Arc<WindowWakeHandleInner> {
        let thread_id = unsafe { GetCurrentThreadId() };

        Arc::new(WindowWakeHandleInner {
            thread_id: Mutex::new(Some(thread_id)),
        })
    }
}
