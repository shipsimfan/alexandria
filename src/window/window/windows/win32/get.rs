use crate::{
    math::{Vector2i, Vector2u},
    window::{Win32Window, WindowProc},
};
use win32::{
    ClientToScreen, GetClientRect, GetDpiForWindow, HWND, POINT, RECT, try_get_last_error,
};

impl<T: WindowProc> Win32Window<T> {
    /// Get the size of the client area of the window
    pub fn get_client_size(&self) -> win32::Result<Vector2u> {
        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect)).map(|_| {
            Vector2u::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32,
            )
        })
    }

    /// Get the position of the top-left corner of the client area of the window
    pub fn get_client_position(&self) -> win32::Result<Vector2i> {
        let mut point = POINT::default();
        try_get_last_error!(ClientToScreen(self.handle, &mut point)).map(|_| point.into())
    }

    /// Get the current content scale factor of the window
    pub fn get_content_scale(&self) -> f32 {
        unsafe { GetDpiForWindow(self.handle) as f32 / 96.0 }
    }

    /// Get the handle of the window
    pub fn handle(&self) -> HWND {
        self.handle
    }
}
