use crate::{
    math::{Vector2i, Vector2u},
    window::{DisplayMode, WindowSubsystem},
};
use std::borrow::Cow;

mod create;
mod get;
mod new;
mod set;

/// A builder for new [`Window`](crate::window::Window)s
pub struct WindowBuilder<UserEvent: 'static + Send> {
    /// The name of the window
    title: Cow<'static, str>,

    /// The size of the window to make
    size: Option<Vector2u>,

    /// The maximum size to allow the window to grow to
    maximum_size: Option<Vector2u>,

    /// The minimum size to allow the window to shrink to
    minimum_size: Option<Vector2u>,

    /// The position to make the window at
    position: Option<Vector2i>,

    /// Should the window be bordered?
    bordered: bool,

    /// Should the window be resizable?
    resizable: bool,

    /// Should the window start hidden?
    hidden: bool,

    /// Start the window minimized
    minimized: bool,

    /// Start the window maximized
    maximized: bool,

    /// Should the window be fullscreen?
    fullscreen: bool,

    /// The fullscreen mode to try and use
    ///
    /// If this is [`None`], the window will be put into borderless fullscreen
    fullscreen_mode: Option<DisplayMode>,

    /// The context to produce the window on
    context: WindowSubsystem<UserEvent>,
}
