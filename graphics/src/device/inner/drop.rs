use crate::device::inner::GraphicsDeviceInner;
use std::ptr::null;

impl Drop for GraphicsDeviceInner {
    fn drop(&mut self) {
        (self.functions.destroy_device)(self.handle, null())
    }
}
