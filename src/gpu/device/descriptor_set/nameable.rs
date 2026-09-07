use crate::gpu::{VulkanDescriptorSet, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanDescriptorSet {
    const OBJECT_TYPE: VkObjectType = VkObjectType::DescriptorSet;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
