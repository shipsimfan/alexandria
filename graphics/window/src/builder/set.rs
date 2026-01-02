use crate::{DisplayMode, WindowBuilder};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl WindowBuilder {
    /// Set the title of the window to be created
    pub fn title<S: Into<Cow<'static, str>>>(&mut self, title: S) -> &mut WindowBuilder {
        self.title = title.into();
        self
    }

    /// Set the window to be created with `size`
    pub fn size(&mut self, size: Vector2u) -> &mut WindowBuilder {
        self.size = Some(size);
        self
    }

    /// Set the window to be created with a default size
    pub fn default_size(&mut self) -> &mut WindowBuilder {
        self.size = None;
        self
    }

    /// Set the display mode to create the window with
    pub fn display_mode(&mut self, display_mode: DisplayMode) -> &mut WindowBuilder {
        self.display_mode = display_mode;
        self
    }
}
