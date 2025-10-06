use crate::{Adapter, DisplayMode};

mod create;
mod new;
mod set;

/// Builds a [`Window`](crate::Window) using provided parameters and sensible defaults otherwise
pub struct WindowBuilder<'a> {
    /// The title of the window
    pub title: &'a str,

    /// The x position of the window
    pub x: Option<i32>,

    /// The y position of the window
    pub y: Option<i32>,

    /// The width of the window
    pub width: Option<u32>,

    /// The height of the window
    pub height: Option<u32>,

    /// Should presents be synchronized with vertical blanks?
    pub vsync: bool,

    /// The mode the window should be displayed with
    pub display_mode: DisplayMode,

    /// The adapter to use for rendering
    pub adapter: Option<&'a mut Adapter>,
}
