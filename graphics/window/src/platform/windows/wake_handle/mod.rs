use std::sync::Mutex;
use win32::DWORD;

mod invalidate;
mod new;
mod wake;

/// A handle which can be used to wake the
/// [`Window::wait_for_message`](crate::Window::wait_for_message) function
pub(crate) struct WindowWakeHandleInner {
    /// The ID of the thread handling the window, wrapped in a lock to prevent racing during
    /// invalidation
    thread_id: Mutex<Option<DWORD>>,
}
