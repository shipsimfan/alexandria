use crate::notify::NotifyInner;
use win32::{ResetEvent, Result, try_get_last_error};

impl NotifyInner {
    /// Reset the notify signal without waiting on it
    pub fn reset(&self) -> Result<()> {
        try_get_last_error!(ResetEvent(self.handle)).map(|_| ())
    }
}
