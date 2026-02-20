use crate::window::{DisplayIter, subsystem::WindowSubsystemInner};
use std::cell::Ref;

impl<'a, UserEvent: 'static + Send> DisplayIter<'a, UserEvent> {
    /// Create a new [`DisplayIter`]
    pub(in crate::window) fn new(
        r#ref: Ref<'a, WindowSubsystemInner<UserEvent>>,
    ) -> DisplayIter<'a, UserEvent> {
        DisplayIter { index: 0, r#ref }
    }
}
