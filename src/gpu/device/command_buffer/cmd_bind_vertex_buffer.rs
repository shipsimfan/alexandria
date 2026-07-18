use crate::gpu::{VulkanBuffer, VulkanCommandBuffer};

impl VulkanCommandBuffer {
    /// Bind a vertex buffer to the command buffer
    pub fn cmd_bind_vertex_buffer(
        &mut self,
        first_binding: u32,
        buffer: &VulkanBuffer,
        offset: u64,
    ) {
        unsafe {
            (self
                .device
                .functions()
                .command_buffer
                .cmd_bind_vertex_buffers)(
                self.handle, first_binding, 1, &buffer.handle(), &offset
            )
        };
    }
}
