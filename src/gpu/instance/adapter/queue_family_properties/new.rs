use crate::gpu::VulkanQueueFamilyProperties;
use vulkan::VkQueueFamilyProperties;

impl VulkanQueueFamilyProperties {
    /// Create a new [`VulkanQueueFamilyProperties`]
    pub(in crate::gpu::instance::adapter) fn new(
        index: u32,
        properties: VkQueueFamilyProperties,
    ) -> VulkanQueueFamilyProperties {
        VulkanQueueFamilyProperties { index, properties }
    }
}
