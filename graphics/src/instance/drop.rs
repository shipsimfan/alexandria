use crate::GraphicsInstance;
use std::ptr::null;

impl Drop for GraphicsInstance {
    fn drop(&mut self) {
        (self.functions.destroy_instance)(self.instance, null())
    }
}
