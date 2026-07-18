use crate::gpu::{VulkanBuffer, VulkanCommandBuffer, VulkanIndexType};

impl VulkanCommandBuffer {
    /// Bind an index buffer to the command buffer
    pub fn cmd_bind_index_buffer(
        &mut self,
        buffer: &VulkanBuffer,
        offset: u64,
        index_type: VulkanIndexType,
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_bind_index_buffer)(
                self.handle,
                buffer.handle(),
                offset,
                index_type,
            )
        };
    }
}
