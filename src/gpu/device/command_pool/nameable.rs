use crate::gpu::{VulkanCommandPool, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanCommandPool {
    const OBJECT_TYPE: VkObjectType = VkObjectType::CommandPool;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
