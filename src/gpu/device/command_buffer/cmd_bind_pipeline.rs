use crate::gpu::{VulkanCommandBuffer, VulkanPipeline, VulkanPipelineBindPoint};

impl VulkanCommandBuffer {
    /// Bind a graphics or compute pipeline to the command buffer
    pub fn cmd_bind_pipeline(
        &mut self,
        bind_point: VulkanPipelineBindPoint,
        pipeline: &VulkanPipeline,
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_bind_pipeline)(
                self.handle,
                bind_point,
                pipeline.handle(),
            )
        };
    }
}
