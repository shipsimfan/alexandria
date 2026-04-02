use crate::gpu::VulkanSwapchain;
use std::ptr::null;

impl<'surface> Drop for VulkanSwapchain<'surface> {
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
