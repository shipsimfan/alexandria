use crate::{WindowWakeHandle, WindowWakeHandleInner};
use std::sync::Arc;

impl WindowWakeHandle {
    /// Create a new [`WindowWakeHandle`] around `inner`
    pub(crate) fn new(inner: Arc<WindowWakeHandleInner>) -> WindowWakeHandle {
        WindowWakeHandle { inner }
    }
}
