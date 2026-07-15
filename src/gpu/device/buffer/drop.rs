use crate::gpu::VulkanBuffer;
use std::ptr::null;

impl Drop for VulkanBuffer {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().buffer.destroy_buffer)(
                self.device.handle(),
                self.handle,
                null(),
            )
        }
    }
}
