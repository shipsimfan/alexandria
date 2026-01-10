use crate::{CursorLock, Window, WindowEvents};

impl<Callbacks: WindowEvents> Drop for Window<Callbacks> {
    fn drop(&mut self) {
        self.wake_handle.invalidate();

        if self.state.cursor_lock() == CursorLock::Locked {
            self.handle.lock_cursor_to_window(false).unwrap();
        }
    }
}
