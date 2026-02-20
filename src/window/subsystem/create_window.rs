use crate::{
    Result,
    window::{Window, WindowBuilder, WindowSubsystem},
};
use std::borrow::Cow;

impl<UserEvent: 'static + Send> WindowSubsystem<UserEvent> {
    /// Create a new [`WindowBuilder`]
    pub fn create_window<S: Into<Cow<'static, str>>>(&self, title: S) -> WindowBuilder<UserEvent> {
        WindowBuilder::new(title.into(), self.clone())
    }

    /// Create a new [`Window`] with the settings of the given builder
    pub(in crate::window) fn do_create_window(
        &self,
        builder: &WindowBuilder<UserEvent>,
    ) -> Result<Window<UserEvent>> {
        self.inner.borrow_mut().create_window(builder)
    }
}
