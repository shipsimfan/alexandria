use crate::gpu::VulkanSampler;
use std::ptr::null;

impl Drop for VulkanSampler {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().sampler.destroy_sampler)(
                self.device.handle(),
                self.handle,
                null(),
            )
        }
    }
}
