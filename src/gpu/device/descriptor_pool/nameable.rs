use crate::gpu::{VulkanDescriptorPool, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanDescriptorPool {
    const OBJECT_TYPE: VkObjectType = VkObjectType::DescriptorPool;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
