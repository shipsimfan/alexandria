use crate::gpu::{VulkanNameable, VulkanPipelineCache};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanPipelineCache {
    const OBJECT_TYPE: VkObjectType = VkObjectType::PipelineCache;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
