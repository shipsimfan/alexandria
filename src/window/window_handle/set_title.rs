use crate::{window::WindowHandle, Result};
use win32::{try_get_last_error, SetWindowText};

impl WindowHandle {
    /// Set the window title
    pub fn set_title(&mut self, title: &[u16]) -> Result<()> {
        assert!(title.len() > 0);
        assert_eq!(title[title.len() - 1], 0);

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))?;
        Ok(())
    }
}
