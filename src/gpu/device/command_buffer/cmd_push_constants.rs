use crate::gpu::{VulkanCommandBuffer, VulkanPipelineLayout, VulkanShaderStageFlags};

impl VulkanCommandBuffer {
    /// Set the values of a range of push constants in a command buffer
    pub fn cmd_push_constants<F: Into<VulkanShaderStageFlags>>(
        &self,
        pipeline_layout: &VulkanPipelineLayout,
        stage_flags: F,
        offset: u32,
        values: &[u8],
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_push_constants)(
                self.handle,
                pipeline_layout.handle(),
                stage_flags.into(),
                offset,
                values.len() as _,
                values.as_ptr().cast(),
            )
        };
    }
}
