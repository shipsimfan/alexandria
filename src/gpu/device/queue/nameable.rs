use crate::gpu::{VulkanNameable, VulkanQueue};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanQueue {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Queue;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
