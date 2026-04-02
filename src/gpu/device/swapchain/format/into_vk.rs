use crate::gpu::VulkanSwapchainFormat;
use vulkan::{VkFormat, khr_surface::VkColorSpaceKhr};

impl VulkanSwapchainFormat {
    /// Convert this format into its Vulkan counter-part
    pub(in crate::gpu::device::swapchain) fn into_vk(self) -> (VkFormat, VkColorSpaceKhr) {
        match self {
            VulkanSwapchainFormat::B8G8R8A8Srgb => {
                (VkFormat::B8G8R8A8UNorm, VkColorSpaceKhr::SRGBNonlinearKhr)
            }
            VulkanSwapchainFormat::R8G8B8A8Srgb => {
                (VkFormat::R8G8B8A8UNorm, VkColorSpaceKhr::SRGBNonlinearKhr)
            }
            VulkanSwapchainFormat::OtherSrgb(format) => {
                (format.into_vk(), VkColorSpaceKhr::SRGBNonlinearKhr)
            }
        }
    }
}
