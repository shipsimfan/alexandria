use crate::gpu::VulkanCommandBuffer;

impl VulkanCommandBuffer {
    /// Draw some vertices to the screen
    pub fn cmd_draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            (self.device.functions().command_buffer.cmd_draw)(
                self.handle,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }
}
