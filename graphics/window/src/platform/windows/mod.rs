//! The Windows implementation of the window system

use crate::{Result, WindowState};
use std::sync::Arc;
use window_class::WindowClass;
use window_handle::WindowHandle;

mod error;
mod wake_handle;
mod window_class;
mod window_handle;

mod builder;
mod deref;
mod drop;
mod get;
mod new;
mod process_messages;
mod set;
mod wait_for_message;
mod window_proc;

pub(crate) use error::OsError;
pub(crate) use wake_handle::WindowWakeHandleInner;

/// A window displayed for the user
pub struct Window {
    /// The result of the window procedure
    wnd_proc_result: Result<()>,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The handle used to wake this thread if blocking for messages
    wake_handle: Arc<WindowWakeHandleInner>,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,

    /// The current state of the window
    state: WindowState,
}
