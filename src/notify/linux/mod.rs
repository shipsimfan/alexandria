use std::ffi::c_int;

mod drop;
mod get;
mod new;
mod notify;
mod reset;
mod wait;

/// The implementation of a [`Notify`](crate::Notify) on Linux
pub(crate) struct NotifyInner {
    /// The handle to the eventfd
    handle: c_int,

    /// Should the notify be reset after waiting?
    auto_reset: bool,
}

unsafe impl Send for NotifyInner {}
unsafe impl Sync for NotifyInner {}
