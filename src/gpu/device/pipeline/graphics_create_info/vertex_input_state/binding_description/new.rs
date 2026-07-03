use crate::gpu::{VulkanVertexInputBindingDescription, VulkanVertexInputRate};
use vulkan::VkVertexInputBindingDescription;

impl VulkanVertexInputBindingDescription {
    /// Creates a new [`VulkanVertexInputBindingDescription`]
    pub fn new(
        binding: u32,
        stride: u32,
        input_rate: VulkanVertexInputRate,
    ) -> VulkanVertexInputBindingDescription {
        VulkanVertexInputBindingDescription {
            inner: VkVertexInputBindingDescription {
                binding,
                stride,
                input_rate,
            },
        }
    }
}
