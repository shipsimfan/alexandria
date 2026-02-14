use crate::window::subsystem::WindowSubsystemInner;
use std::cell::Ref;

mod iterator;
mod new;

/// An iterator over [`Display`](crate::window::Display)s in the window subsystem
pub struct DisplayIter<'a> {
    /// The index of the next display to provide
    index: usize,

    /// The reference to the window subsystem to give each display
    r#ref: Ref<'a, WindowSubsystemInner>,
}
