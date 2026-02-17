use crate::{Notify, Result, window::WindowSubsystem};
use std::time::Duration;

impl WindowSubsystem {
    /// Wait for an event to occur on the window subsystem, `notify` signals, or `timeout` passes
    pub(crate) fn wait_for_event(
        &self,
        notify: &Notify,
        timeout: Option<Duration>,
    ) -> Result<bool> {
        self.inner.borrow().wait_for_event(notify, timeout)
    }
}
