use crate::notify::NotifyInner;
use win32::{SetEvent, try_get_last_error};

impl NotifyInner {
    /// Notify any waiting threads on this notify
    ///
    /// If this is an auto-reset notify, then only a single waiting thread will be released
    pub fn notify(&self) -> win32::Result<()> {
        try_get_last_error!(SetEvent(self.handle)).map(|_| ())
    }
}
