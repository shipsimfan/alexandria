use crate::window::subsystem::WindowSubsystemInner;
use std::cell::Ref;

mod iter;

#[cfg(target_os = "windows")]
mod windows;

mod get;
mod inner;
mod new;

pub use iter::DisplayIter;

#[cfg(target_os = "windows")]
pub(in crate::window) use windows::DisplayInner;

/// An active display for presenting images to a user
pub struct Display<'a> {
    /// The index of the display in `r#ref`
    index: usize,

    /// The reference to the window subsystem containing this display
    r#ref: Ref<'a, WindowSubsystemInner>,
}
