use crate::window::{WindowClass, WindowProc};
use std::rc::Rc;
use win32::HWND;

mod deref;
mod drop;
mod new;

/// A raw Win32 window
pub(in crate::window) struct Win32Window<T: WindowProc> {
    /// The handle to the window
    handle: HWND,

    /// The data passed to the window procedure
    user_data: Box<T>,

    /// The class this window belongs to
    #[allow(unused)]
    class: Rc<WindowClass<T>>,
}
