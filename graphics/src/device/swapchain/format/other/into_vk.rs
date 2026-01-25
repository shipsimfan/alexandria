use crate::device::swapchain::format::OtherSwapchainFormat;
use vulkan::VkFormat;

impl OtherSwapchainFormat {
    /// Convert this format into its Vulkan counter-part
    pub(in crate::device::swapchain::format) fn into_vk(self) -> VkFormat {
        self.format
    }
}
