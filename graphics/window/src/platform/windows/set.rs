use crate::{CursorLock, DisplayMode, Result, Window};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl Window {
    /// Set the title of the window to `title`
    pub fn set_title<S: Into<Cow<'static, str>>>(&mut self, title: S) -> Result<()> {
        let title = title.into();

        // Convert the title to UTF-16
        let mut title_utf16: Vec<_> = title.encode_utf16().chain([0]).collect();

        // Set the window title
        self.handle.set_title(&title_utf16)?;

        // Save the title
        self.state.set_title(title);

        Ok(())
    }

    /// Set the size of the window
    pub fn set_size(&mut self, size: Vector2u) -> Result<()> {
        self.handle.set_size(self.display_mode(), size)
    }

    /// Set the display mode of the window
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) -> Result<()> {
        self.handle.set_display_mode(display_mode)?;
        self.state.set_display_mode(display_mode);
        Ok(())
    }

    /// Set how the cursor should be locked to the window
    pub fn set_cursor_lock(&mut self, cursor_lock: CursorLock) -> Result<()> {
        self.handle
            .lock_cursor_to_window(cursor_lock == CursorLock::Locked && self.is_focused())?;
        self.state.set_cursor_lock(cursor_lock);
        Ok(())
    }

    /// Request the window to close
    pub fn request_close(&mut self) {
        self.state.set_is_close_requested(true);
    }
}
