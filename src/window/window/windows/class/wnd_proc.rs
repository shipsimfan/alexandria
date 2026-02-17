use win32::{DefWindowProc, GWLP_USERDATA, GetWindowLongPtr, HWND, LPARAM, LRESULT, UINT, WPARAM};

/// An item that can be used as the user data for a window procedure
pub(in crate::window) trait WindowProc {
    /// Called when a message is pumped on the given window
    ///
    /// Return `false` to have the default window procedure run for the message
    fn wnd_proc(this: Option<&mut Self>, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> bool;
}

/// The window procedure trampoline for all windows
pub(in crate::window::window::windows::class) unsafe extern "system" fn wnd_proc_trampoline<
    T: WindowProc,
>(
    wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let window_ptr = unsafe { GetWindowLongPtr(wnd, GWLP_USERDATA) };
    let window = if window_ptr == 0 {
        None
    } else {
        Some(unsafe { &mut *(window_ptr as *mut T) })
    };

    if !T::wnd_proc(window, msg, w_param, l_param) {
        unsafe { DefWindowProc(wnd, msg, w_param, l_param) }
    } else {
        0
    }
}
