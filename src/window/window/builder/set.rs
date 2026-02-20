use crate::{
    math::{Vector2i, Vector2u},
    window::{DisplayMode, WindowBuilder},
};
use std::borrow::Cow;

impl<UserEvent: 'static + Send> WindowBuilder<UserEvent> {
    /// Set the name of the window
    pub fn title<S: Into<Cow<'static, str>>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Set the size of the window to make
    ///
    /// If this is [`None`], the window will be created with a default size depending on the
    /// platform
    ///
    /// [`None`] is the default
    pub fn size(&mut self, size: Option<Vector2u>) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the maximum size to allow the window to grow to
    ///
    /// If this is [`None`], the window will be able to grow without limit
    ///
    /// [`None`] is the default
    pub fn maximum_size(&mut self, maximum_size: Option<Vector2u>) -> &mut Self {
        self.maximum_size = maximum_size;
        self
    }

    /// Set the minimum size to allow the window to shrink to
    ///
    /// If this is [`None`], the window will be able to shrink to a size of 1x1
    ///
    /// [`None`] is the default
    pub fn minimum_size(&mut self, minimum_size: Option<Vector2u>) -> &mut Self {
        self.minimum_size = minimum_size;
        self
    }

    /// Set the position to make the window at
    ///
    /// If this is [`None`], the window will be automatically positioned by the system
    ///
    /// [`None`] is the default
    pub fn position(&mut self, position: Option<Vector2i>) -> &mut Self {
        self.position = position;
        self
    }

    /// Set that the window should be bordered
    pub fn bordered(&mut self) -> &mut Self {
        self.bordered = true;
        self
    }

    /// Set that the window should be borderless
    ///
    /// This is the default
    pub fn borderless(&mut self) -> &mut Self {
        self.bordered = false;
        self
    }

    /// Set that the window should be resizable
    pub fn resizable(&mut self) -> &mut Self {
        self.resizable = true;
        self
    }

    /// Set that the window should not be resizable
    ///
    /// This is the default
    pub fn non_resizable(&mut self) -> &mut Self {
        self.resizable = false;
        self
    }

    /// Set that the window should start hidden
    pub fn hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }

    /// Set that the window should start visible
    ///
    /// This is the default
    pub fn visible(&mut self) -> &mut Self {
        self.hidden = false;
        self
    }

    /// Set that the window should start minimized
    pub fn minimized(&mut self) -> &mut Self {
        self.minimized = true;
        self
    }

    /// Set that the window should start not minimized
    ///
    /// This is the default
    pub fn not_minimized(&mut self) -> &mut Self {
        self.minimized = false;
        self
    }

    /// Set that the window should start maximized
    pub fn maximized(&mut self) -> &mut Self {
        self.maximized = true;
        self
    }

    /// Set that the window should start not maximized
    ///
    /// This is the default
    pub fn not_maximized(&mut self) -> &mut Self {
        self.maximized = false;
        self
    }

    /// Set that the window should be fullscreen
    pub fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen = true;
        self
    }

    /// Set that the window should not be fullscreen
    ///
    /// This is the default
    pub fn not_fullscreen(&mut self) -> &mut Self {
        self.fullscreen = false;
        self
    }

    /// Set the fullscreen mode to try and use
    ///
    /// If this is [`None`], the window will be put into borderless fullscreen
    pub fn fullscreen_mode(&mut self, fullscreen_mode: Option<DisplayMode>) -> &mut Self {
        self.fullscreen_mode = fullscreen_mode;
        self
    }
}
