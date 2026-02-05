use crate::notify::NotifyInner;
use linux::{Result, try_linux, unistd::write};

impl NotifyInner {
    /// Notify any waiting threads on this notify
    ///
    /// If this is an auto-reset notify, then only a single waiting thread will be released
    pub fn notify(&self) -> Result<()> {
        let buf = 1u64.to_ne_bytes();
        try_linux!(write(self.handle, buf.as_ptr().cast(), buf.len() as _)).map(|_| ())
    }
}
