use crate::gpu::{VulkanCommandBuffer, VulkanDebugUtilsLabel};

impl<'a> VulkanDebugUtilsLabel<'a> {
    /// Get the command buffer associated with this debug utils label
    pub fn command_buffer(&mut self) -> &mut VulkanCommandBuffer {
        self.command_buffer
    }
}
