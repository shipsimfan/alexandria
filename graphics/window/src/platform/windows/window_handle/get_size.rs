use crate::{Result, WindowError, platform::windows::WindowHandle};
use alexandria_math::Vector2u;
use win32::{GetClientRect, RECT, try_get_last_error};

impl WindowHandle {
    /// Gets the current size of the client area of the window
    pub fn get_size(&self) -> Result<Vector2u> {
        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect))
            .map_err(|os| WindowError::new_os("unable to get window size", os))?;

        Ok(Vector2u::new(
            (rect.right - rect.left) as _,
            (rect.bottom - rect.top) as _,
        ))
    }
}
