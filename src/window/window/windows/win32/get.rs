use crate::{
    math::Vector2i,
    window::{Win32Window, WindowProc},
};
use win32::{ClientToScreen, GetClientRect, POINT, RECT, try_get_last_error};

impl<T: WindowProc> Win32Window<T> {
    /// Get the size of the client area of the window
    pub fn get_client_size(&self) -> win32::Result<Vector2i> {
        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect))
            .map(|_| Vector2i::new(rect.right - rect.left, rect.bottom - rect.top))
    }

    /// Get the position of the top-left corner of the client area of the window
    pub fn get_client_position(&self) -> win32::Result<Vector2i> {
        let mut point = POINT::default();
        try_get_last_error!(ClientToScreen(self.handle, &mut point)).map(|_| point.into())
    }
}
