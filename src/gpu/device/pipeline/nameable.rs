use crate::gpu::{VulkanNameable, VulkanPipeline};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanPipeline {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Pipeline;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
