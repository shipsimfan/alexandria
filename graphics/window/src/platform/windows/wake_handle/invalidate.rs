use crate::WindowWakeHandleInner;

impl WindowWakeHandleInner {
    /// Invalidate the thread ID held inisde this wake handle
    pub(in crate::platform::windows) fn invalidate(&self) {
        *self.thread_id.lock().unwrap() = None;
    }
}
