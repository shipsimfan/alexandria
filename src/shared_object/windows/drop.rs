use crate::shared_object::SharedObjectInner;
use win32::FreeLibrary;

impl Drop for SharedObjectInner {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.handle) };
    }
}
