use crate::gpu::{VulkanNameable, VulkanShaderModule};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanShaderModule {
    const OBJECT_TYPE: VkObjectType = VkObjectType::ShaderModule;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
