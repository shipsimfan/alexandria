use crate::{DisplayMode, WindowBuilder};
use std::borrow::Cow;

impl WindowBuilder {
    /// Create a new [`WindowBuilder`]
    pub fn new<S: Into<Cow<'static, str>>>(title: S) -> WindowBuilder {
        WindowBuilder {
            title: title.into(),
            size: None,
            display_mode: DisplayMode::default(),
            #[cfg(target_os = "linux")]
            force_x11: false,
        }
    }
}
