use win32::HANDLE;

mod drop;
mod get;
mod new;
mod notify;
mod reset;
mod wait;

/// The implementation of a [`Notify`](crate::Notify) on Windows
pub(crate) struct NotifyInner {
    /// The handle to the win32 event
    handle: HANDLE,
}

unsafe impl Send for NotifyInner {}
unsafe impl Sync for NotifyInner {}
