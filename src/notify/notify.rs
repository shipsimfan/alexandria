use crate::{Error, Notify, Result};

impl Notify {
    /// Notify any waiting threads on this notify
    ///
    /// If this is an auto-reset notify, then only a single waiting thread will be released
    pub fn notify(&self) -> Result<()> {
        self.inner
            .notify()
            .map_err(|os| Error::new_with("unable to notify", os))
    }
}
