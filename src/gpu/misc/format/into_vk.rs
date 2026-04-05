use crate::gpu::VulkanFormat;
use vulkan::{VkFormat, khr_surface::VkColorSpaceKhr};

impl VulkanFormat {
    /// Convert this format into its Vulkan surface counter-part
    pub(in crate::gpu) fn into_vk_surface_format(self) -> (VkFormat, VkColorSpaceKhr) {
        (self.into_vk(), VkColorSpaceKhr::SRGBNonlinearKhr)
    }

    /// Convert this format into its Vulkan image counter-part
    pub(in crate::gpu) fn into_vk(self) -> VkFormat {
        match self {
            VulkanFormat::B8G8R8A8UNorm => VkFormat::B8G8R8A8UNorm,
            VulkanFormat::R8G8B8A8UNorm => VkFormat::R8G8B8A8UNorm,
        }
    }
}
