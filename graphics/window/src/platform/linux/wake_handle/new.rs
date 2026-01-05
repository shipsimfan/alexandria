use crate::WindowWakeHandleInner;
use std::sync::Arc;

impl WindowWakeHandleInner {
    /// Create a new [`WindowWakeHandleInner`]
    pub(in crate::platform::linux) fn new() -> Arc<WindowWakeHandleInner> {
        Arc::new(WindowWakeHandleInner {})
    }
}
