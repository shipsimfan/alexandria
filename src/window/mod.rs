use window_class::WindowClass;
use window_handle::WindowHandle;

mod window_class;
mod window_handle;

mod get;
mod new;
mod process_inputs;
mod window_proc;

/// A window which can be rendered into and receive input
pub struct Window {
    /// Is the window still running?
    is_running: bool,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,
}
