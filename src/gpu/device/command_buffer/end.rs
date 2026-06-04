use crate::{Error, Result, gpu::VulkanCommandBuffer};
use vulkan::try_vulkan;

impl VulkanCommandBuffer {
    /// End recording commands into this command buffer
    pub fn end(&mut self) -> Result<()> {
        try_vulkan!((self.device.functions().command_buffer.end_command_buffer)(
            self.handle
        ))
        .map_err(|vk| Error::new_with("unable to end command buffer", vk))?;

        Ok(())
    }
}
