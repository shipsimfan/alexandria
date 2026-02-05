use crate::notify::NotifyInner;
use win32::CloseHandle;

impl Drop for NotifyInner {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
