use crate::gpu::{VulkanNameable, VulkanRenderPass};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanRenderPass {
    const OBJECT_TYPE: VkObjectType = VkObjectType::RenderPass;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
