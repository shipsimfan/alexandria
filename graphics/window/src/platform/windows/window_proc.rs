use crate::Window;
use alexandria_math::Vector2;
use win32::{
    DefWindowProc, GWLP_USERDATA, GetWindowLongPtr, HWND, LPARAM, LRESULT, UINT, WM_ACTIVATEAPP,
    WM_CLOSE, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_MOVE, WM_SIZE, WPARAM,
};

impl Window {
    /// Called to establish [`Window::window_proc`] as the main window procedure
    pub(in crate::platform::windows) extern "system" fn init_window_proc(
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
    pub(in crate::platform::windows) fn window_proc(
        &mut self,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            // The window is closing or the app is quiting
            WM_CLOSE => self.state.set_is_running(false),

            // The user has begun moving or resizing the window
            WM_ENTERSIZEMOVE => self.state.set_is_changing(true),

            // The user has stopped moving or resizing the window
            WM_EXITSIZEMOVE => self.state.set_is_changing(false),

            // The window has changed size
            WM_SIZE => {
                let width = (l_param & 0xFFFF) as u32;
                let height = ((l_param >> 16) & 0xFFFF) as u32;
                self.state.set_size(Vector2::new(width, height));
            }

            // The window has moved
            WM_MOVE => {
                let x = (l_param & 0xFFFF) as i16;
                let y = ((l_param >> 16) & 0xFFFF) as i16;
                self.state.set_position(Vector2::new(x as _, y as _));
            }

            // The window either gained or lost focus
            WM_ACTIVATEAPP => self.state.set_is_focused(w_param != 0),

            // All other events
            _ => return unsafe { DefWindowProc(*self.handle, msg, w_param, l_param) },
        }

        0
    }
}
