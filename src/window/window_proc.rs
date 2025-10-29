use crate::{
    math::{Vector2i, Vector2u},
    Window,
};
use win32::{
    DefWindowProc, GetWindowLongPtr, GWLP_USERDATA, HWND, LPARAM, LRESULT, UINT, WM_ACTIVATEAPP,
    WM_CLOSE, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_MOVE, WM_QUIT, WM_SIZE, WPARAM,
};

impl<LogCallbacks: crate::LogCallbacks> Window<LogCallbacks> {
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
            unsafe { &mut *(window_ptr as *mut Window<LogCallbacks>) }
                .window_proc(msg, w_param, l_param)
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
            // The window is closing or the app is quiting
            WM_CLOSE | WM_QUIT => self.is_running = false,

            // The user has begun moving or resizing the window
            WM_ENTERSIZEMOVE => {
                self.in_move = true;
            }

            // The user has stopped moving or resizing the window
            WM_EXITSIZEMOVE => {
                self.in_move = false;

                self.wnd_proc_result = self.render_context.resize(
                    &self.graphics_context,
                    self.size,
                    &mut self.log_callbacks,
                );
            }

            // The window has changed size
            WM_SIZE => {
                let width = (l_param & 0xFFFF) as u32;
                let height = ((l_param >> 16) & 0xFFFF) as u32;
                self.size = Vector2u::new(width, height);

                if !self.in_move {
                    self.wnd_proc_result = self.render_context.resize(
                        &self.graphics_context,
                        self.size,
                        &mut self.log_callbacks,
                    );
                }
            }

            // The window has moved
            WM_MOVE => {
                let x = (l_param & 0xFFFF) as i16;
                let y = ((l_param >> 16) & 0xFFFF) as i16;
                self.position = Vector2i::new(x as _, y as _);
            }

            // The window either gained or lost focus
            WM_ACTIVATEAPP => self.is_focused = w_param != 0,

            // All other events
            _ => return unsafe { DefWindowProc(*self.handle, msg, w_param, l_param) },
        }

        0
    }
}
