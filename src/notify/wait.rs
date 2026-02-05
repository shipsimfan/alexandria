use crate::{Error, Notify, Result};
use std::time::Duration;

impl Notify {
    /// Wait for the notify to become signalled
    ///
    /// This function returns `Ok(false)` if `timeout` is reached before the Notify is signalled
    ///
    /// If this is an auto-reset event, only a single thread will be released for each signal and
    /// the signal state will be reset afterwards
    pub fn wait(&self, timeout: Option<Duration>) -> Result<bool> {
        self.inner
            .wait(timeout.map(|timeout| timeout.as_millis() as _))
            .map_err(|os| Error::new_with("unable to wait for a notify", os))
    }
}
