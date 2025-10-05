use crate::{DisplayMode, WindowBuilder};

impl<'a> WindowBuilder<'a> {
    /// Create a new [`WindowBuilder`] with default settings
    pub fn new(title: &'a str) -> Self {
        WindowBuilder {
            title,
            x: None,
            y: None,
            width: None,
            height: None,
            display_mode: DisplayMode::default(),
            adapter: None,
        }
    }
}
