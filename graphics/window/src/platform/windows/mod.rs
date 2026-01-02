//! The Windows implementation of the window system

use crate::{Result, WindowState};
use window_class::WindowClass;
use window_handle::WindowHandle;

mod error;
mod get;
mod new;
mod process_messages;
mod wait_for_message;
mod window_class;
mod window_handle;
mod window_proc;

mod deref;

pub(crate) use error::OsError;

/// A window displayed for the user
pub struct Window {
    /// The result of the window procedure
    wnd_proc_result: Result<()>,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,

    /// The current state of the window
    state: WindowState,
}
