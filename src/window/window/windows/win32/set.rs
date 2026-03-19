use crate::{
    Error, Result,
    math::Vector2i,
    window::{Win32Window, WindowProc, WindowStyle},
};
use std::ptr::null_mut;
use win32::{
    GWL_EXSTYLE, GWL_STYLE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_SHOW, SWP_FRAMECHANGED,
    SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREPOSITION, SWP_NOSIZE, SWP_NOZORDER, SetWindowLong,
    SetWindowPos, SetWindowText, ShowWindow, try_get_last_error,
};

impl<T: WindowProc> Win32Window<T> {
    /// Sets the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let title: Vec<_> = title.encode_utf16().chain([0]).collect();

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))
            .map_err(|os| Error::new_with("unable to set window title", os))?;
        Ok(())
    }

    /// Sets the position of the window
    pub fn set_position(&mut self, position: Vector2i) -> Result<()> {
        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            position.x,
            position.y,
            0,
            0,
            SWP_NOREPOSITION | SWP_NOSIZE | SWP_NOOWNERZORDER | SWP_NOZORDER
        ))
        .map_err(|os| Error::new_with("unable to set a window's position", os))
        .map(|_| ())
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

    /// Maximize the window
    pub fn maximize(&mut self) -> Result<()> {
        try_get_last_error!(ShowWindow(self.handle, SW_MAXIMIZE))
            .map_err(|os| Error::new_with("unable to set the window maximized", os))
            .map(|_| ())
    }

    /// Minimize the window
    pub fn minimize(&mut self) -> Result<()> {
        try_get_last_error!(ShowWindow(self.handle, SW_MINIMIZE))
            .map_err(|os| Error::new_with("unable to set the window minized", os))
            .map(|_| ())
    }

    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        try_get_last_error!(ShowWindow(self.handle, SW_HIDE))
            .map_err(|os| Error::new_with("unable to set the window hidden", os))
            .map(|_| ())
    }

    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        try_get_last_error!(ShowWindow(self.handle, SW_SHOW))
            .map_err(|os| Error::new_with("unable to set the window shown", os))
            .map(|_| ())
    }

    /// Set the style of the window
    pub fn set_style(&mut self, style: WindowStyle) -> Result<()> {
        try_get_last_error!(SetWindowLong(self.handle, GWL_STYLE, style.style as _))
            .map_err(|os| Error::new_with("unable to set window border", os))?;
        try_get_last_error!(SetWindowLong(self.handle, GWL_EXSTYLE, style.ex_style as _))
            .map_err(|os| Error::new_with("unable to set window border", os))?;

        try_get_last_error!(SetWindowPos(
            self.handle,
            null_mut(),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER | SWP_NOZORDER | SWP_FRAMECHANGED
        ))
        .map_err(|os| Error::new_with("unable to set window border", os))
        .map(|_| ())
    }
}
