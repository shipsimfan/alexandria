use crate::gpu::device::VulkanDeviceInner;
use std::ptr::null;

impl Drop for VulkanDeviceInner {
    fn drop(&mut self) {
        unsafe { (self.functions.destroy_device)(self.handle, null()) }
    }
}
