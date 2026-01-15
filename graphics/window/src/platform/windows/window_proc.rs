use crate::{CursorLock, Window, WindowEvents};
use alexandria_math::{Vector2, Vector2u};
use win32::{
    DefWindowProc, GWLP_USERDATA, GetWindowLongPtr, HWND, LPARAM, LRESULT, SIZE_MAXIMIZED,
    SIZE_MINIMIZED, SIZE_RESTORED, UINT, WM_ACTIVATEAPP, WM_CLOSE, WM_ENTERSIZEMOVE,
    WM_EXITSIZEMOVE, WM_MOVE, WM_SIZE, WPARAM,
};

impl<Callbacks: WindowEvents> Window<Callbacks> {
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
            unsafe { &mut *(window_ptr as *mut Window<Callbacks>) }
                .window_proc(msg, w_param, l_param)
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
            WM_CLOSE => {
                self.state.set_is_close_requested(true);
                self.callbacks.on_close_requested();
            }

            // The user has begun moving or resizing the window
            WM_ENTERSIZEMOVE => self.is_resizing = Some(self.size()),

            // The user has stopped moving or resizing the window
            WM_EXITSIZEMOVE => {
                if let Some(size) = self.is_resizing.take() {
                    self.on_resize(size);
                }
            }

            // The window has changed size
            WM_SIZE => {
                let width = (l_param & 0xFFFF) as u32;
                let height = ((l_param >> 16) & 0xFFFF) as u32;
                if width != 0 && height != 0 {
                    let size = Vector2::new(width, height);
                    if self.is_resizing.is_none() {
                        self.on_resize(size);
                    } else {
                        self.is_resizing = Some(size);
                    }
                }

                match w_param {
                    SIZE_MAXIMIZED => {
                        self.state.set_is_maximized(true);
                        self.callbacks.on_maximized();
                    }
                    SIZE_RESTORED | SIZE_MINIMIZED => {
                        self.state.set_is_maximized(false);
                        self.callbacks.on_restored();
                    }
                    _ => {}
                }
            }

            WM_MOVE => {
                if self.cursor_lock() == CursorLock::Locked && self.is_focused() {
                    self.wnd_proc_result = self.handle.lock_cursor_to_window(true);
                }
            }

            // The window either gained or lost focus
            WM_ACTIVATEAPP => {
                self.state.set_is_focused(w_param != 0);
                self.callbacks.on_focus_change(self.state.is_focused());
                if self.cursor_lock() == CursorLock::Locked {
                    self.wnd_proc_result = self.handle.lock_cursor_to_window(self.is_focused());
                }
            }

            // All other events
            _ => return unsafe { DefWindowProc(*self.handle, msg, w_param, l_param) },
        }

        0
    }

    fn on_resize(&mut self, size: Vector2u) {
        if size == self.size() {
            return;
        }

        self.state.set_size(size);
        self.callbacks.on_resize(size);

        if self.cursor_lock() == CursorLock::Locked && self.is_focused() {
            self.wnd_proc_result = self.handle.lock_cursor_to_window(true);
        }
    }
}
