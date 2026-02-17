use std::marker::PhantomData;
use win32::ATOM;
use wnd_proc::wnd_proc_trampoline;

mod wnd_proc;

mod drop;
mod get;
mod register;

pub(in crate::window) use wnd_proc::WindowProc;

/// A Win32 window class
pub(in crate::window) struct WindowClass<T: WindowProc> {
    /// The handle to the window class
    handle: ATOM,

    /// A marker for the type of the user data passed to the window procedure
    _user_data: PhantomData<T>,
}
