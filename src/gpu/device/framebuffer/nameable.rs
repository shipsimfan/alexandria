use crate::gpu::{VulkanFramebuffer, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanFramebuffer {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Framebuffer;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
