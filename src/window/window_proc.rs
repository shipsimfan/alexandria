use crate::Window;
use win32::{
    DefWindowProc, GetWindowLongPtr, GWLP_USERDATA, HWND, LPARAM, LRESULT, UINT, WM_CLOSE, WM_QUIT,
    WPARAM,
};

impl Window {
    /// Called to establish [`Window::window_proc`] as the main window procedure
    pub(in crate::window) extern "system" fn init_window_proc(
        wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        let window_ptr = unsafe { GetWindowLongPtr(wnd, GWLP_USERDATA) };
        if window_ptr == 0 {
            unsafe { DefWindowProc(wnd, msg, w_param, l_param) }
        } else {
            unsafe { &mut *(window_ptr as *mut Window) }.window_proc(msg, w_param, l_param)
        }
    }

    /// Called when an event is consumed by the message pump
    pub(in crate::window) fn window_proc(
        &mut self,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CLOSE | WM_QUIT => {
                self.is_running = false;
                0
            }
            _ => unsafe { DefWindowProc(*self.handle, msg, w_param, l_param) },
        }
    }
}
