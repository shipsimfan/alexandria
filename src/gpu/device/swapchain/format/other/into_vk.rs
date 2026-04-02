use crate::gpu::device::swapchain::format::OtherVulkanSwapchainFormat;
use vulkan::VkFormat;

impl OtherVulkanSwapchainFormat {
    /// Convert this format into its Vulkan counter-part
    pub(in crate::gpu::device::swapchain::format) fn into_vk(self) -> VkFormat {
        self.format
    }
}
