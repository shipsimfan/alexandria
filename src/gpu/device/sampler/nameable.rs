use crate::gpu::{VulkanNameable, VulkanSampler};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanSampler {
    const OBJECT_TYPE: VkObjectType = VkObjectType::Sampler;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
