use crate::{DisplayMode, Result, WindowError, platform::windows::WindowHandle};
use std::ptr::null_mut;
use win32::{
    GWL_EXSTYLE, GWL_STYLE, GetLastError, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    SetLastError, SetWindowLong, SetWindowPos, try_get_last_error,
};

impl WindowHandle {
    /// Sets the display mode of the window
    #[allow(unused)]
    pub fn set_display_mode(&self, display_mode: DisplayMode) -> Result<()> {
        let (style, ex_style) = display_mode.style();

        unsafe { SetLastError(0) };
        if unsafe { SetWindowLong(self.handle, GWL_STYLE, style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(WindowError::new_os(
                    "unable to set window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        if unsafe { SetWindowLong(self.handle, GWL_EXSTYLE, ex_style as _) } == 0 {
            if unsafe { GetLastError() != 0 } {
                return Err(WindowError::new_os(
                    "unable to set extended window style",
                    win32::Error::get_last_error(),
                ));
            }
        }

        let mut flags = SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED;
        if display_mode != DisplayMode::Borderless {
            flags |= SWP_NOMOVE;
        }

        try_get_last_error!(SetWindowPos(self.handle, null_mut(), 0, 0, 0, 0, flags))
            .map_err(|error| WindowError::new_os("unable to set window display mode", error))?;

        Ok(())
    }
}
