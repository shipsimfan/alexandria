use crate::{
    math::{Vector2i, Vector2u},
    window::WindowHandle,
    DisplayMode, Error, Result,
};
use std::ptr::null_mut;
use win32::{
    try_get_last_error, GetLastError, SetLastError, SetWindowLong, SetWindowPos, GWL_EXSTYLE,
    GWL_STYLE,
};

impl WindowHandle {
    /// Sets the display mode, position, and size of the window
    pub(in crate::window) fn set_display_mode_size_and_position(
        &self,
        display_mode: DisplayMode,
        size: Vector2u,
        position: Vector2i,
    ) -> Result<()> {
        let (size, position) = display_mode.client_to_window(size, position)?;
        let (style, ex_style) = display_mode.style();

        unsafe { SetLastError(0) };
        if unsafe { SetWindowLong(self.handle, GWL_STYLE, style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(Error::new_os(
                    "unable to set window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        if unsafe { SetWindowLong(self.handle, GWL_EXSTYLE, ex_style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(Error::new_os(
                    "unable to set extended window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            position.x,
            position.y,
            size.x as _,
            size.y as _,
            0
        ))
        .map_err(|os| Error::new_os("unable to set window position and size", os))?;

        Ok(())
    }
}
