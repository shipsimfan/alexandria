use crate::{Result, WindowError, platform::windows::WindowHandle};
use win32::{SetWindowText, try_get_last_error};

impl WindowHandle {
    /// Set the window title
    #[allow(unused)]
    pub fn set_title(&mut self, title: &[u16]) -> Result<()> {
        assert!(title.len() > 0);
        assert_eq!(title[title.len() - 1], 0);

        try_get_last_error!(SetWindowText(self.handle, title.as_ptr()))
            .map_err(|os| WindowError::new_os("unable to set window title", os))?;
        Ok(())
    }
}
