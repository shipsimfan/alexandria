use crate::{
    graphics::{GraphicsContext, RenderContext},
    math::{Vector2i, Vector2u},
    Result,
};
use window_class::WindowClass;

mod builder;
mod display_mode;
mod window_class;
mod window_handle;

mod begin_render;
mod get;
mod get_debug_messages;
mod new;
mod process_inputs;
mod set;
mod window_proc;

pub use builder::WindowBuilder;
pub use display_mode::DisplayMode;

pub(crate) use window_handle::WindowHandle;

/// A window which can be rendered into and receive input
pub struct Window<LogCallbacks: crate::LogCallbacks = ()> {
    /// Is the window still running?
    is_running: bool,

    /// The position of the upper-left corner of the client area of the window
    position: Vector2i,

    /// The size of the client area of the window
    size: Vector2u,

    /// Should presents be synchronized with vertical blanks?
    vsync: bool,

    /// The current display mode of the window
    display_mode: DisplayMode,

    /// Is this window the one being focused on?
    is_focused: bool,

    /// Is the window being actively moved or resized?
    in_move: bool,

    /// The context used for rendering
    render_context: RenderContext,

    /// The context used for creating graphics objects
    graphics_context: GraphicsContext,

    /// The callback functions for logging events
    log_callbacks: LogCallbacks,

    /// The result of the window procedure
    wnd_proc_result: Result<()>,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,
}
