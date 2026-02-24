use crate::window::subsystem::WindowSubsystemInner;
use std::cell::Ref;

mod iter;
mod orientation;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod get;
mod inner;
mod new;

pub use iter::DisplayIter;
pub use orientation::DisplayOrientation;

#[cfg(target_os = "linux")]
pub(in crate::window) use linux::DisplayInner;
#[cfg(target_os = "windows")]
pub(in crate::window) use windows::DisplayInner;

/// An active display for presenting images to a user
pub struct Display<'a, UserEvent: 'static + Send> {
    /// The index of the display in `r#ref`
    index: usize,

    /// The reference to the window subsystem containing this display
    r#ref: Ref<'a, WindowSubsystemInner<UserEvent>>,
}
