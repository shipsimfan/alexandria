use crate::window::{Display, subsystem::WindowSubsystemInner};
use std::cell::Ref;

impl<'a, UserEvent: 'static + Send> Display<'a, UserEvent> {
    /// Create a new [`Display`]
    pub(in crate::window) fn new(
        index: usize,
        r#ref: Ref<'a, WindowSubsystemInner<UserEvent>>,
    ) -> Display<'a, UserEvent> {
        Display { index, r#ref }
    }
}
