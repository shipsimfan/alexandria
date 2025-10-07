use std::ptr::null_mut;

use crate::{
    math::{Vector2i, Vector2u},
    window::WindowHandle,
    Result,
};
use win32::{try_get_last_error, AdjustWindowRectEx, SetWindowPos, FALSE, RECT};

impl WindowHandle {
    /// Sets the position and size of the window
    pub fn set_size_and_position(&self, size: Vector2u, position: Vector2i) -> Result<()> {
        let mut rect = RECT {
            left: position.x,
            top: position.y,
            right: (size.x as i32) + position.x,
            bottom: (size.y as i32) + position.y,
        };

        try_get_last_error!(AdjustWindowRectEx(
            &mut rect,
            self.style,
            FALSE,
            self.ex_style
        ))?;

        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            rect.left,
            rect.top,
            rect.right - rect.left,
            rect.bottom - rect.top,
            0
        ))?;

        Ok(())
    }
}
