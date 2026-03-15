use crate::{
    Error, Result,
    math::Vector2i,
    window::{Win32Window, WindowProc},
};
use std::ptr::null_mut;
use win32::{
    SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREPOSITION, SWP_NOZORDER, SetWindowPos, SetWindowText,
    try_get_last_error,
};

impl<T: WindowProc> Win32Window<T> {
    /// Sets the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let title: Vec<_> = title.encode_utf16().chain([0]).collect();

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))
            .map_err(|os| Error::new_with("unable to set window title", os))?;
        Ok(())
    }

    /// Sets the size of the client area of the window
    pub fn set_size(&mut self, size: Vector2i) -> Result<()> {
        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            0,
            0,
            size.x,
            size.y,
            SWP_NOREPOSITION | SWP_NOMOVE | SWP_NOOWNERZORDER | SWP_NOZORDER
        ))
        .map_err(|os| Error::new_with("unable to set a window's size", os))
        .map(|_| ())
    }
}
