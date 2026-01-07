use crate::instance::GraphicsInstanceInner;
use std::ptr::null;

impl Drop for GraphicsInstanceInner {
    fn drop(&mut self) {
        (self.functions.destroy_instance)(self.handle, null())
    }
}
