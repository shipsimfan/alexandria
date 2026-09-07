use crate::gpu::{VulkanBuffer, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanBuffer {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Buffer;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
