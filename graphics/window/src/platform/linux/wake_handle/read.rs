use crate::{Result, WindowError, WindowWakeHandleInner};

impl WindowWakeHandleInner {
    /// Read the event count from the wake handle, clearing it
    pub(in crate::platform::linux) fn read(&self) -> Result<()> {
        self.event
            .read()
            .map(|_| ())
            .map_err(|os| WindowError::new_os("unable to clear wake handle count", os))
    }
}
