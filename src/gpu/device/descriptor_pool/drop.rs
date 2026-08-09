use crate::gpu::VulkanDescriptorPool;
use std::ptr::null;

impl Drop for VulkanDescriptorPool {
    fn drop(&mut self) {
        unsafe {
            (self
                .device
                .functions()
                .descriptor_pool
                .destroy_descriptor_pool)(self.device.handle(), self.handle, null())
        };
    }
}
