use crate::gpu::device::swapchain::format::OtherVulkanSwapchainFormat;
use vulkan::VkFormat;

impl OtherVulkanSwapchainFormat {
    /// Create a new [`OtherSwapchainFormat`]
    pub(in crate::gpu::device::swapchain::format) fn new(format: VkFormat) -> OtherVulkanSwapchainFormat {
        OtherVulkanSwapchainFormat { format }
    }
}
