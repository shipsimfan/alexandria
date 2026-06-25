use crate::gpu::VulkanDescriptorSetLayout;
use vulkan::VkDescriptorSetLayout;

impl VulkanDescriptorSetLayout {
    /// Get the raw Vulkan descriptor set layout handle
    pub(in crate::gpu::device) fn handle(&self) -> VkDescriptorSetLayout {
        self.handle
    }
}
