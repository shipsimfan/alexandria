use crate::window::{WindowBuilder, WindowSubsystem};
use std::borrow::Cow;

impl WindowSubsystem {
    /// Create a new [`WindowBuilder`]
    pub fn create_window<S: Into<Cow<'static, str>>>(&self, title: S) -> WindowBuilder {
        WindowBuilder::new(title.into(), self.clone())
    }
}
