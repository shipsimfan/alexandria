use crate::notify::NotifyInner;
use linux::unistd::close;

impl Drop for NotifyInner {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
