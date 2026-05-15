use crate::{Error, Result, gpu::VulkanFence};
use vulkan::try_vulkan;

impl VulkanFence {
    /// Waits for the fence to be signaled
    pub fn wait(&self, timeout: u64) -> Result<()> {
        try_vulkan!((self.device.functions().fence.wait_for_fences)(
            self.device.handle(),
            1,
            &self.handle,
            0,
            timeout,
        ))
        .map_err(|vk| Error::new_with("unable to wait on a fence", vk))?;

        Ok(())
    }
}
