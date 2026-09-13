use crate::{
    Error, Result,
    gpu::{VulkanFence, VulkanResult},
};
use vulkan::{VK_TRUE, try_vulkan};

impl VulkanFence {
    /// Waits for the fence to be signaled
    pub fn wait(&self, timeout: u64) -> Result<VulkanResult> {
        try_vulkan!((self.device.functions().fence.wait_for_fences)(
            self.device.handle(),
            1,
            &self.handle,
            VK_TRUE,
            timeout,
        ))
        .map_err(|vk| Error::new_with("unable to wait on a fence", vk))
    }
}
