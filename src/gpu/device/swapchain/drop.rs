use crate::gpu::VulkanSwapchain;
use std::ptr::null;

impl Drop for VulkanSwapchain {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().swapchain().destroy_swapchain)(
                self.device.handle(),
                self.handle,
                null(),
            )
        };
    }
}
