use crate::notify::NotifyInner;
use win32::{DWORD, Error, INFINITE, Result, WAIT_OBJECT_0, WaitForSingleObject};

impl NotifyInner {
    /// Wait for the notify to become signalled
    ///
    /// This function returns `Ok(false)` if `timeout` is reached before the Notify is signalled
    ///
    /// If this is an auto-reset event, only a single thread will be released for each signal and
    /// the signal state will be reset afterwards
    pub fn wait(&self, timeout_ms: Option<DWORD>) -> Result<bool> {
        const WAIT_TIMEOUT: DWORD = win32::WAIT_TIMEOUT as _;

        match unsafe { WaitForSingleObject(self.handle, timeout_ms.unwrap_or(INFINITE)) } {
            WAIT_OBJECT_0 => Ok(true),
            WAIT_TIMEOUT => Ok(false),
            _ => Err(Error::get_last_error()),
        }
    }
}
