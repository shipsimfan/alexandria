use std::sync::Arc;

#[cfg(target_os = "linux")]
use linux::NotifyInner;
#[cfg(target_os = "windows")]
use windows::NotifyInner;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod get;
mod new;
mod notify;
mod reset;
mod wait;

/// An item that can be waited on until it is notified by another thread
#[derive(Clone)]
pub struct Notify {
    /// The implementation of the notify mechanism
    inner: Arc<NotifyInner>,
}
