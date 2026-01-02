use crate::WindowWakeHandleInner;
use win32::{PostThreadMessage, WM_APP, try_get_last_error};

impl WindowWakeHandleInner {
    /// Wake the window thread if it is blocking
    pub fn wake(&self) {
        let thread_id = self.thread_id.lock().unwrap();
        if let Some(thread_id) = *thread_id {
            try_get_last_error!(PostThreadMessage(thread_id, WM_APP, 0, 0)).unwrap();
        }
    }
}
