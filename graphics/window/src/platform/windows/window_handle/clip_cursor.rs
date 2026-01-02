use crate::{Result, WindowError, platform::windows::WindowHandle};
use std::ptr::null;
use win32::{ClientToScreen, ClipCursor, GetClientRect, POINT, RECT, try_get_last_error};

impl WindowHandle {
    /// Lock the cursor to stay in the window area
    pub fn lock_cursor_to_window(&self, enable: bool) -> Result<()> {
        if !enable {
            try_get_last_error!(ClipCursor(null()))
                .map_err(|os| WindowError::new_os("unable to free cursor lock", os))?;
            return Ok(());
        }

        let mut rect = RECT::default();
        try_get_last_error!(GetClientRect(self.handle, &mut rect))
            .map_err(|os| WindowError::new_os("unable to get window rect", os))?;

        let mut tl = POINT {
            x: rect.left,
            y: rect.top,
        };
        let mut br = POINT {
            x: rect.right,
            y: rect.bottom,
        };

        try_get_last_error!(ClientToScreen(self.handle, &mut tl)).map_err(|os| {
            WindowError::new_os("unable to convert top-left window position to screen", os)
        })?;
        try_get_last_error!(ClientToScreen(self.handle, &mut br)).map_err(|os| {
            WindowError::new_os(
                "unable to convert bottom-right window position to screen",
                os,
            )
        })?;

        rect = RECT {
            left: tl.x,
            top: tl.y,
            right: br.x,
            bottom: br.y,
        };
        try_get_last_error!(ClipCursor(&rect))
            .map_err(|os| WindowError::new_os("unable to lock cursor to window", os))?;
        Ok(())
    }
}
