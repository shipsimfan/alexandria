use crate::window::{WindowIter, subsystem::WindowSubsystemInner};
use std::cell::Ref;

impl<'a, UserEvent: 'static + Send> WindowIter<'a, UserEvent> {
    /// Create a new [`WindowIter`]
    pub(in crate::window) fn new(
        r#ref: Ref<'a, WindowSubsystemInner<UserEvent>>,
    ) -> WindowIter<'a, UserEvent> {
        WindowIter { index: 0, r#ref }
    }
}
