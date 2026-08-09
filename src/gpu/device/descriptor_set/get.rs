use crate::gpu::VulkanDescriptorSet;
use vulkan::VkDescriptorSet;

impl VulkanDescriptorSet {
    /// Gets the raw Vulkan command pool handle
    pub(in crate::gpu::device) fn handle(&self) -> VkDescriptorSet {
        self.handle
    }
}
