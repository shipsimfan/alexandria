use crate::gpu::device::command_pool::VulkanCommandPoolInner;
use std::ptr::null;

impl Drop for VulkanCommandPoolInner {
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
