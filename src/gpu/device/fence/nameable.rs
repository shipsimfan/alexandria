use crate::gpu::{VulkanFence, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanFence {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Fence;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
