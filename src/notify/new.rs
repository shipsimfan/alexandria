use crate::{Error, Notify, Result, notify::NotifyInner};
use std::sync::Arc;

impl Notify {
    /// Create a new [`Notify`]
    ///
    /// If `auto_reset` is `true`, then when notified, this will release only a single thread
    /// before reseting the signal and returning from `wait`
    pub fn new(auto_reset: bool, initial_state: bool) -> Result<Notify> {
        NotifyInner::new(auto_reset, initial_state)
            .map(|inner| Notify {
                inner: Arc::new(inner),
            })
            .map_err(|os| Error::new_with("unable to create a notify", os))
    }
}
