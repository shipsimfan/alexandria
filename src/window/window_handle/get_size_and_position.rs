use std::ptr::null_mut;

use crate::{
    math::{Vector2i, Vector2u},
    window::WindowHandle,
    Result,
};
use win32::{
    try_get_last_error, GetClientRect, GetLastError, MapWindowPoints, SetLastError, RECT, SUCCEEDED,
};

impl WindowHandle {
    /// Gets the current position and size of the window
    pub fn get_size_and_position(&self) -> Result<(Vector2i, Vector2u)> {
        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect))?;

        let size = Vector2u::new(rect.right as _, rect.bottom as _);

        unsafe { SetLastError(0) };
        unsafe { MapWindowPoints(self.handle, null_mut(), &mut rect as *mut RECT as _, 2) };
        if !SUCCEEDED!(unsafe { GetLastError() }) {
            return Err(win32::Error::get_last_error().into());
        }

        let position = Vector2i::new(rect.left, rect.top);

        Ok((position, size))
    }
}
