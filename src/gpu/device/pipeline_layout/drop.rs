use crate::gpu::VulkanPipelineLayout;
use std::ptr::null;

impl Drop for VulkanPipelineLayout {
    fn drop(&mut self) {
        unsafe {
            (self
                .device
                .functions()
                .pipeline_layout
                .destroy_pipeline_layout)(self.device.handle(), self.handle, null())
        };
    }
}
