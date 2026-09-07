use crate::gpu::{VulkanCommandBuffer, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanCommandBuffer {
    const OBJECT_TYPE: VkObjectType = VkObjectType::CommandBuffer;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
