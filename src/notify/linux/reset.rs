use crate::notify::NotifyInner;
use linux::{Error, Result, try_linux, unistd::read};

impl NotifyInner {
    /// Reset the notify signal without waiting on it
    pub fn reset(&self) -> Result<()> {
        self.try_reset().map(|_| ())
    }

    /// Try to reset the event if it isn't already
    ///
    /// Returns `true` if the event was reset
    pub(in crate::notify::linux) fn try_reset(&self) -> Result<bool> {
        let mut buffer = [0; 8];
        match try_linux!(read(
            self.handle,
            buffer.as_mut_ptr().cast(),
            buffer.len() as _
        )) {
            Ok(_) => Ok(true),
            Err(Error::EAGAIN) => Ok(false),
            Err(error) => Err(error),
        }
    }
}
