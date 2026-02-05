use crate::{Error, Notify, Result};

impl Notify {
    /// Reset the notify signal without waiting on it
    pub fn reset(&self) -> Result<()> {
        self.inner
            .reset()
            .map_err(|os| Error::new_with("unable to reset a notify", os))
    }
}
