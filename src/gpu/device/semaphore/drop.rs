use crate::gpu::VulkanSemaphore;
use std::ptr::null;

impl Drop for VulkanSemaphore {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().semaphore.destroy_semaphore)(
                self.device.handle(),
                self.handle,
                null(),
            );
        }
    }
}
