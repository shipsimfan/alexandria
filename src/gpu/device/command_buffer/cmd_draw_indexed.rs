use crate::gpu::VulkanCommandBuffer;

impl VulkanCommandBuffer {
    /// Draw some indexed vertices to the screen
    pub fn cmd_draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_draw_indexed)(
                self.handle,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }
}
