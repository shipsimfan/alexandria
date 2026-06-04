use crate::{Error, Result, gpu::VulkanCommandBuffer};
use vulkan::{VkCommandBufferBeginInfo, try_vulkan};

impl VulkanCommandBuffer {
    /// Start recording commands into this command buffer
    pub fn begin(&mut self) -> Result<()> {
        let begin_info = VkCommandBufferBeginInfo {
            ..Default::default()
        };

        try_vulkan!((self
            .device
            .functions()
            .command_buffer
            .begin_command_buffer)(self.handle, &begin_info))
        .map_err(|vk| Error::new_with("unable to begin command buffer", vk))?;

        Ok(())
    }
}
