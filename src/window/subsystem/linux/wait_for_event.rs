use crate::{Notify, Result, window::subsystem::WindowSubsystemInner};
use std::time::Duration;

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Wait for an event to occur on the window subsystem, `notify` signals, or `timeout` passes
    pub(in crate::window::subsystem) fn wait_for_event(
        &self,
        notify: &Notify,
        timeout: Option<Duration>,
    ) -> Result<bool> {
        notify.wait(timeout)
    }
}
