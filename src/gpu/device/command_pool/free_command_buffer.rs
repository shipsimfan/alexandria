use crate::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};

impl VulkanCommandPool {
    /// Frees a command buffer back to this command pool
    pub fn free_command_buffer(&mut self, id: Id<VulkanCommandBuffer>) {
        if let Some(command_buffer) = self.command_buffers.remove(id) {
            unsafe {
                (self.device.functions().command_buffer.free_command_buffers)(
                    self.device.handle(),
                    self.handle,
                    1,
                    &command_buffer.handle(),
                )
            };
        }
    }
}
