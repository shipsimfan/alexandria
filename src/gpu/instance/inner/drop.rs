use crate::gpu::instance::VulkanInstanceInner;
use std::ptr::null;

impl Drop for VulkanInstanceInner {
    fn drop(&mut self) {
        unsafe { (self.functions.destroy_instance)(self.handle, null()) }
    }
}
