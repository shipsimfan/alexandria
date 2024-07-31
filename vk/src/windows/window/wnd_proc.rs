use super::Window;
use std::ptr::null_mut;
use win32::{
    DefWindowProc, GetWindowLongPtr, PostQuitMessage, GWLP_USERDATA, HWND, LPARAM, LRESULT, UINT,
    WPARAM,
};

/// The window procedure entry point which will call the window procedure on the [`Window`]
/// associated with `wnd`
pub(super) unsafe extern "system" fn wnd_proc(
    wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let this: *mut Window = GetWindowLongPtr(wnd, GWLP_USERDATA) as _;

    if this == null_mut() {
        return DefWindowProc(wnd, msg, w_param, l_param);
    }

    (*this).wnd_proc(msg, w_param, l_param)
}

impl Window {
    /// The window procedure which handles messages from the operating system
    pub(self) fn wnd_proc(&mut self, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        match msg {
            win32::WM_CLOSE => unsafe { PostQuitMessage(0) },
            _ => return unsafe { DefWindowProc(self.wnd, msg, w_param, l_param) },
        }

        0
    }
}
