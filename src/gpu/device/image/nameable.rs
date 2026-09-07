use crate::gpu::{VulkanImage, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanImage {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Image;

    fn handle(&self) -> u64 {
        self.handle().as_u64()
    }
}
