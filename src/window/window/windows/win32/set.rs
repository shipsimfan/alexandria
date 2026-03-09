use win32::{SetWindowText, try_get_last_error};

use crate::{
    Error, Result,
    window::{Win32Window, WindowProc},
};

impl<T: WindowProc> Win32Window<T> {
    /// Sets the title of the window
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        let title: Vec<_> = title.encode_utf16().chain([0]).collect();

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))
            .map_err(|os| Error::new_with("unable to set window title", os))?;
        Ok(())
    }
}
