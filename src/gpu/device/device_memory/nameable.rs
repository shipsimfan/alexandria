use crate::gpu::{VulkanDeviceMemory, VulkanNameable};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanDeviceMemory {
    const OBJECT_TYPE: VkObjectType = VkObjectType::DeviceMemory;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
