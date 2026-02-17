use crate::window::{Display, subsystem::WindowSubsystemInner};
use std::cell::Ref;

impl<'a> Display<'a> {
    /// Create a new [`Display`]
    pub(in crate::window) fn new(
        index: usize,
        r#ref: Ref<'a, WindowSubsystemInner>,
    ) -> Display<'a> {
        Display { index, r#ref }
    }
}
