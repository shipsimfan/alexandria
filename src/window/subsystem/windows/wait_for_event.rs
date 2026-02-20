use crate::{Error, Notify, Result, window::subsystem::WindowSubsystemInner};
use std::time::Duration;
use win32::{
    DWORD, FALSE, INFINITE, MsgWaitForMultipleObjects, QS_ALLINPUT, WAIT_OBJECT_0, WAIT_TIMEOUT,
};

impl<UserEvent: 'static + Send> WindowSubsystemInner<UserEvent> {
    /// Wait for an event to occur on the window subsystem, `notify` signals, or `timeout` passes
    pub(in crate::window::subsystem) fn wait_for_event(
        &self,
        notify: &Notify,
        timeout: Option<Duration>,
    ) -> Result<bool> {
        let handles = [unsafe { notify.inner().handle() }];
        let result = unsafe {
            MsgWaitForMultipleObjects(
                handles.len() as _,
                handles.as_ptr(),
                FALSE,
                timeout
                    .map(|timeout| timeout.as_millis() as _)
                    .unwrap_or(INFINITE),
                QS_ALLINPUT,
            )
        };

        if result == WAIT_OBJECT_0 {
            notify.reset()?;
            return Ok(true);
        }

        if result == WAIT_OBJECT_0 + handles.len() as DWORD {
            return Ok(true);
        }

        if result == WAIT_TIMEOUT as _ {
            return Ok(false);
        }

        Err(Error::new_with(
            "unable to wait for an event",
            win32::Error::get_last_error(),
        ))
    }
}
