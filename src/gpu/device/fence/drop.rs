use crate::gpu::VulkanFence;
use std::ptr::null;

impl Drop for VulkanFence {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().fence.destroy_fence)(
                self.device.handle(),
                self.handle,
                null(),
            );
        }
    }
}
