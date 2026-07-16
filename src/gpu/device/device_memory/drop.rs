use crate::gpu::VulkanDeviceMemory;
use std::ptr::null;

impl Drop for VulkanDeviceMemory {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().device_memory.free_memory)(
                self.device.handle(),
                self.handle,
                null(),
            );
        }
    }
}
