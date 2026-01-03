use crate::{Window, WindowBuilder};
use std::borrow::Cow;

impl Window {
    /// Create a new [`WindowBuilder`]
    pub fn builder<S: Into<Cow<'static, str>>>(title: S) -> WindowBuilder {
        WindowBuilder::new(title)
    }
}
