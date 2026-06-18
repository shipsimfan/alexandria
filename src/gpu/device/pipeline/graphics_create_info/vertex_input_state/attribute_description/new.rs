use crate::gpu::{VulkanFormat, VulkanVertexInputAttributeDescription};
use vulkan::VkVertexInputAttributeDescription;

impl VulkanVertexInputAttributeDescription {
    /// Creates a new [`VulkanVertexInputAttributeDescription`]
    pub fn new(
        location: u32,
        binding: u32,
        format: VulkanFormat,
        offset: u32,
    ) -> VulkanVertexInputAttributeDescription {
        VulkanVertexInputAttributeDescription {
            inner: VkVertexInputAttributeDescription {
                location,
                binding,
                format,
                offset,
            },
        }
    }
}
