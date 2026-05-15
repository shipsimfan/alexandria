use crate::{Error, Result, gpu::VulkanFence};
use vulkan::try_vulkan;

impl VulkanFence {
    /// Resets the fence to the unsignaled state
    pub fn reset(&mut self) -> Result<()> {
        try_vulkan!((self.device.functions().fence.reset_fences)(
            self.device.handle(),
            1,
            &self.handle,
        ))
        .map_err(|vk| Error::new_with("unable to reset a fence", vk))?;

        Ok(())
    }
}
