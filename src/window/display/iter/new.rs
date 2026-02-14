use crate::window::{DisplayIter, subsystem::WindowSubsystemInner};
use std::cell::Ref;

impl<'a> DisplayIter<'a> {
    /// Create a new [`DisplayIter`]
    pub(in crate::window) fn new(r#ref: Ref<'a, WindowSubsystemInner>) -> DisplayIter<'a> {
        DisplayIter { index: 0, r#ref }
    }
}
