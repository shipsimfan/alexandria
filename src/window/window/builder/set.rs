use crate::{
    math::{Vector2i, Vector2u},
    window::{DisplayMode, WindowBuilder},
};
use std::borrow::Cow;

impl WindowBuilder {
    /// Set the name of the window
    pub fn title<S: Into<Cow<'static, str>>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Set the name of the window
    pub fn set_title<S: Into<Cow<'static, str>>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Set the size of the window to make
    pub fn size(mut self, size: Option<Vector2u>) -> Self {
        self.size = size;
        self
    }

    /// Set the size of the window to make
    pub fn set_size(&mut self, size: Option<Vector2u>) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the maximum size to allow the window to grow to
    pub fn maximum_size(mut self, maximum_size: Option<Vector2u>) -> Self {
        self.maximum_size = maximum_size;
        self
    }

    /// Set the maximum size to allow the window to grow to
    pub fn set_maximum_size(&mut self, maximum_size: Option<Vector2u>) -> &mut Self {
        self.maximum_size = maximum_size;
        self
    }

    /// Set the minimum size to allow the window to shrink to
    pub fn minimum_size(mut self, minimum_size: Option<Vector2u>) -> Self {
        self.minimum_size = minimum_size;
        self
    }

    /// Set the minimum size to allow the window to shrink to
    pub fn set_minimum_size(&mut self, minimum_size: Option<Vector2u>) -> &mut Self {
        self.minimum_size = minimum_size;
        self
    }

    /// Set the position to make the window at
    pub fn position(mut self, position: Option<Vector2i>) -> Self {
        self.position = position;
        self
    }

    /// Set the position to make the window at
    pub fn set_position(&mut self, position: Option<Vector2i>) -> &mut Self {
        self.position = position;
        self
    }

    /// Set that the window should be bordered
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }

    /// Set that the window should be borderless
    pub fn borderless(mut self) -> Self {
        self.bordered = false;
        self
    }

    /// Set that the window should be resizable
    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    /// Set that the window should not be resizable
    pub fn non_resizable(mut self) -> Self {
        self.resizable = false;
        self
    }

    /// Set that the window should start hidden
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Set that the window should start visible
    ///
    /// This is the default
    pub fn visible(mut self) -> Self {
        self.hidden = false;
        self
    }

    /// Set that the window should start minimized
    pub fn minimized(mut self) -> Self {
        self.minimized = true;
        self
    }

    /// Set that the window should start not minimized
    ///
    /// This is the default
    pub fn not_minimized(mut self) -> Self {
        self.minimized = false;
        self
    }

    /// Set that the window should start maximized
    pub fn maximized(mut self) -> Self {
        self.maximized = true;
        self
    }

    /// Set that the window should be fullscreen
    pub fn fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }

    /// Set the fullscreen mode to try and use
    ///
    /// If this is [`None`], the window will be put into borderless fullscreen
    pub fn fullscreen_mode(mut self, fullscreen_mode: Option<DisplayMode>) -> Self {
        self.fullscreen_mode = fullscreen_mode;
        self
    }
}
