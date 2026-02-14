use crate::{
    Error, Result,
    window::{display::DisplayInner, subsystem::WindowSubsystemInner},
};
use win32::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext, try_get_last_error,
};

impl WindowSubsystemInner {
    /// Create a new [`WindowSubsystemInner`]
    pub(in crate::window::subsystem) fn new() -> Result<WindowSubsystemInner> {
        // Set the DPI awareness
        try_get_last_error!(SetProcessDpiAwarenessContext(
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
        ))
        .map_err(|os| Error::new_with("unable to set DPI awareness", os))?;

        // Enumerate the available displays
        let displays = DisplayInner::enumerate()?.into_iter().collect();

        Ok(WindowSubsystemInner { displays })
    }
}
