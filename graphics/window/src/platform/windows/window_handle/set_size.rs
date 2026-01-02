use crate::{DisplayMode, Result, WindowError, platform::windows::WindowHandle};
use alexandria_math::{Vector2i, Vector2u};
use std::ptr::null_mut;
use win32::{SWP_NOMOVE, SWP_NOZORDER, SetWindowPos, try_get_last_error};

impl WindowHandle {
    /// Sets the size of the window
    #[allow(unused)]
    pub fn set_size(&self, display_mode: DisplayMode, size: Vector2u) -> Result<()> {
        let (style, ex_style) = display_mode.style();

        let size = display_mode.client_to_window(size)?;
        let size = Vector2i::new(size.x as _, size.y as _);

        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            0,
            0,
            size.x,
            size.y,
            SWP_NOMOVE | SWP_NOZORDER
        ))
        .map_err(|error| WindowError::new_os("unable to set window size", error))?;

        Ok(())
    }
}
