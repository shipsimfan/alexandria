use crate::gpu::{VulkanDescriptorPoolSize, VulkanDescriptorType};
use vulkan::VkDescriptorPoolSize;

impl VulkanDescriptorPoolSize {
    /// Create a new [`VulkanDescriptorPoolSize`]
    pub fn new(r#type: VulkanDescriptorType, count: u32) -> VulkanDescriptorPoolSize {
        VulkanDescriptorPoolSize {
            inner: VkDescriptorPoolSize {
                r#type,
                descriptor_count: count,
            },
        }
    }
}
