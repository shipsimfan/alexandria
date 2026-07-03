use crate::gpu::VulkanPipeline;
use std::ptr::null;

impl Drop for VulkanPipeline {
    fn drop(&mut self) {
        unsafe {
            (self.device.functions().pipeline.destroy_pipeline)(
                self.device.handle(),
                self.handle,
                null(),
            )
        }
    }
}
