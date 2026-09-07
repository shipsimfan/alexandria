use crate::gpu::{VulkanNameable, VulkanSemaphore};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanSemaphore {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Semaphore;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
