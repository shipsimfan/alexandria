use crate::{Notify, notify::NotifyInner};

impl Notify {
    /// Get a reference to the inner implementation
    pub(crate) fn inner(&self) -> &NotifyInner {
        &self.inner
    }
}
