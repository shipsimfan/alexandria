use crate::DisplayMode;
use alexandria_math::Vector2u;
use std::borrow::Cow;

mod build;
mod get;
mod new;
mod set;

/// A builder struct for [`Window`](crate::Window)s
pub struct WindowBuilder {
    /// The title of the window to create
    title: Cow<'static, str>,

    /// The size of the window to create
    size: Option<Vector2u>,

    /// The display mode for the window
    display_mode: DisplayMode,

    /// Force the use of X11 over Wayland on Linux
    #[cfg(target_os = "linux")]
    force_x11: bool,
}
