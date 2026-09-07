use crate::gpu::{VulkanDescriptorSetLayout, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanDescriptorSetLayout {
    const OBJECT_TYPE: VkObjectType = VkObjectType::DescriptorSetLayout;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
