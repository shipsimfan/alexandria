use crate::window::subsystem::WindowSubsystemInner;
use std::cell::Ref;

mod iterator;
mod new;

/// An iterator over [`Window`](crate::window::Window)s in the window subsystem
pub struct WindowIter<'a, UserEvent: 'static + Send> {
    /// The index of the next window to provide
    index: usize,

    /// The reference to the window subsystem to give each window
    r#ref: Ref<'a, WindowSubsystemInner<UserEvent>>,
}
