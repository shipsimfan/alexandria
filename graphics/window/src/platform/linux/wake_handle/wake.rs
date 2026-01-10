use crate::{Result, WindowError, WindowWakeHandleInner};

impl WindowWakeHandleInner {
    /// Wake the window thread if it is blocking
    pub fn wake(&self) -> Result<()> {
        self.event
            .signal()
            .map_err(|os| WindowError::new_os("unable to signal main thread", os))
    }
}
