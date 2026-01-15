use crate::device::swapchain::format::OtherSwapchainFormat;
use vulkan::VkFormat;

impl OtherSwapchainFormat {
    /// Create a new [`OtherSwapchainFormat`]
    pub(in crate::device::swapchain::format) fn new(format: VkFormat) -> OtherSwapchainFormat {
        OtherSwapchainFormat { format }
    }
}
