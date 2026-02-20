use crate::window::{WindowBuilder, WindowSubsystem};
use std::borrow::Cow;

impl<UserEvent: 'static + Send> WindowBuilder<UserEvent> {
    /// Create a new [`WindowBuilder`]
    pub(in crate::window) fn new(
        title: Cow<'static, str>,
        context: WindowSubsystem<UserEvent>,
    ) -> WindowBuilder<UserEvent> {
        WindowBuilder {
            title,
            size: None,
            maximum_size: None,
            minimum_size: None,
            position: None,
            bordered: true,
            resizable: false,
            hidden: false,
            minimized: false,
            maximized: false,
            fullscreen: false,
            fullscreen_mode: None,
            context,
        }
    }
}
