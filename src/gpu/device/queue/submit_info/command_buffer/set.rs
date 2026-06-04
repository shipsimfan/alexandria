use crate::gpu::{VulkanCommandBuffer, VulkanCommandBufferSubmitInfo};

impl<'a> VulkanCommandBufferSubmitInfo<'a> {
    /// Set the command buffer being submitted
    pub fn set_command_buffer(mut self, command_buffer: &'a VulkanCommandBuffer) -> Self {
        self.inner.command_buffer = command_buffer.handle();
        self
    }

    /// Set the device mask for this command buffer submit info
    pub fn set_device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
