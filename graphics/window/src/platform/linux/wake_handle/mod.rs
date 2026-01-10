use linux::EventFd;

mod get_fd;
mod new;
mod read;
mod wake;

/// A handle which can be used to wake the
/// [`Window::process_messages`](crate::Window::process_messages) function
pub(crate) struct WindowWakeHandleInner {
    /// The event to signal to wake up the main thread
    event: EventFd,
}
