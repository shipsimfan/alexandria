use crate::gpu::{VulkanNameable, VulkanPipelineLayout};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanPipelineLayout {
    const OBJECT_TYPE: VkObjectType = VkObjectType::PipelineLayout;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
