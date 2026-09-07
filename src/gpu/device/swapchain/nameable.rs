use crate::gpu::{VulkanNameable, VulkanSwapchain};
use vulkan::VkObjectType;

impl<'surface> VulkanNameable for VulkanSwapchain<'surface> {
    const OBJECT_TYPE: VkObjectType = VkObjectType::SwapchainKhr;

    fn handle(&self) -> u64 {
        self.handle.as_u64()
    }
}
