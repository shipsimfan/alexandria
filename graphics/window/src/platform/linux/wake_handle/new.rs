use crate::{Result, WindowError, WindowWakeHandleInner};
use linux::{EventFd, sys::eventfd::EFD_NONBLOCK};
use std::sync::Arc;

impl WindowWakeHandleInner {
    /// Create a new [`WindowWakeHandleInner`]
    pub(in crate::platform::linux) fn new() -> Result<Arc<WindowWakeHandleInner>> {
        let event = EventFd::new(0, EFD_NONBLOCK)
            .map_err(|os| WindowError::new_os("unable to make window wake signal", os))?;

        Ok(Arc::new(WindowWakeHandleInner { event }))
    }
}
