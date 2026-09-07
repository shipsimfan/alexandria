use crate::gpu::{VulkanImageView, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanImageView {
    const OBJECT_TYPE: VkObjectType = VkObjectType::ImageView;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
