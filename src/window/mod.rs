use crate::{
    math::{Vector2i, Vector2u},
    Result,
};
use window_class::WindowClass;
use window_handle::WindowHandle;

mod builder;
mod display_mode;
mod graphics_context;
mod window_class;
mod window_handle;

mod end_render;
mod get;
mod new;
mod process_inputs;
mod window_proc;

pub use builder::WindowBuilder;
pub use display_mode::DisplayMode;
pub use graphics_context::GraphicsContext;

/// A window which can be rendered into and receive input
pub struct Window {
    /// Is the window still running?
    is_running: bool,

    /// The position of the upper-left corner of the client area of the window
    position: Vector2i,

    /// The size of the client area of the window
    size: Vector2u,

    /// The context used for rendering
    graphics_context: GraphicsContext,

    /// The result of the window procedure
    wnd_proc_result: Result<()>,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,
}
