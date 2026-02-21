use crate::notify::NotifyInner;
use linux::{
    Result,
    poll::{POLLIN, poll, pollfd},
    try_linux,
};
use std::ffi::c_int;

impl NotifyInner {
    /// Wait for the notify to become signalled
    ///
    /// This function returns `Ok(false)` if `timeout` is reached before the Notify is signalled
    ///
    /// If this is an auto-reset event, only a single thread will be released for each signal and
    /// the signal state will be reset afterwards
    pub fn wait(&self, timeout_ms: Option<c_int>) -> Result<bool> {
        loop {
            let mut fds = [pollfd {
                fd: self.handle,
                events: POLLIN as _,
                revents: 0,
            }];

            match try_linux!(poll(
                fds.as_mut_ptr(),
                fds.len() as _,
                timeout_ms.unwrap_or(-1)
            )) {
                Ok(0) => return Ok(false),
                Ok(_) => {}
                Err(linux::Error::EINTR) => continue,
                Err(error) => return Err(error),
            }

            if !self.auto_reset {
                return Ok(true);
            }

            if self.try_reset()? {
                return Ok(true);
            }
        }
    }
}
