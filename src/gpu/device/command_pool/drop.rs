use crate::gpu::device::VulkanCommandPool;
use std::ptr::null;

impl Drop for VulkanCommandPool {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().command_pool.destroy_command_pool)(
                self.device.handle(),
                self.handle,
                null(),
            )
        }
    }
}
