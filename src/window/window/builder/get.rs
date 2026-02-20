use crate::{
    math::{Vector2i, Vector2u},
    window::{DisplayMode, WindowBuilder, WindowSubsystem},
};
use std::borrow::Cow;

impl<UserEvent: 'static + Send> WindowBuilder<UserEvent> {
    /// Get the name of the window
    pub fn get_title(&self) -> &Cow<'static, str> {
        &self.title
    }

    /// Get the size of the window to make
    pub fn get_size(&self) -> Option<Vector2u> {
        self.size
    }

    /// Get the maximum size to allow the window to grow to
    pub fn get_maximum_size(&self) -> Option<Vector2u> {
        self.maximum_size
    }

    /// Get the minimum size to allow the window to shrink to
    pub fn get_minimum_size(&self) -> Option<Vector2u> {
        self.minimum_size
    }

    /// Get the position to make the window at
    pub fn get_position(&self) -> Option<Vector2i> {
        self.position
    }

    /// Should the window be bordered?
    pub fn is_bordered(&self) -> bool {
        self.bordered
    }

    /// Should the window be resizable?
    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    /// Should the window start hidden?
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Start the window minimized
    pub fn is_minimized(&self) -> bool {
        self.minimized
    }

    /// Start the window maximized
    pub fn is_maximized(&self) -> bool {
        self.maximized
    }

    /// Should the window be fullscreen?
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Get the fullscreen mode to try and use
    ///
    /// If this is [`None`], the window will be put into borderless fullscreen
    pub fn get_fullscreen_mode(&self) -> Option<DisplayMode> {
        self.fullscreen_mode
    }

    /// Get the context to produce the window on
    pub fn get_context(&self) -> &WindowSubsystem<UserEvent> {
        &self.context
    }
}
