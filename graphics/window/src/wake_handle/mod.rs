use crate::WindowWakeHandleInner;
use std::sync::Arc;

mod new;
mod wake;

/// A handle which can be used to wake the
/// [`Window::wait_for_message`](crate::Window::wait_for_message) function
pub struct WindowWakeHandle {
    /// A reference to the actual wake handle
    inner: Arc<WindowWakeHandleInner>,
}
