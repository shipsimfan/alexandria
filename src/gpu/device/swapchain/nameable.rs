use crate::gpu::{VulkanNameable, VulkanSwapchain};
use vulkan::VkObjectType;

impl VulkanNameable for VulkanSwapchain {
    const OBJECT_TYPE: VkObjectType = VkObjectType::SwapchainKhr;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
