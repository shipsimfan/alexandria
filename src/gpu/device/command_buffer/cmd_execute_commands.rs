use crate::gpu::VulkanCommandBuffer;

impl VulkanCommandBuffer {
    /// Execute a secondary command buffer within this primary command buffer
    pub fn cmd_execute_commands(&mut self, command_buffer: &mut VulkanCommandBuffer) {
        unsafe {
            (self.device.functions().command_buffer.cmd_execute_commands)(
                self.handle,
                1,
                &command_buffer.handle,
            );
        }
    }
}
