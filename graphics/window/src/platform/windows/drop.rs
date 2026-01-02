use crate::Window;

impl Drop for Window {
    fn drop(&mut self) {
        self.wake_handle.invalidate();
    }
}
