use crate::gpu::VulkanCommandBufferSubmitInfo;

impl<'a> VulkanCommandBufferSubmitInfo<'a> {
    /// Get the device mask
    pub fn device_mask(&self) -> u32 {
        self.inner.device_mask
    }
}
