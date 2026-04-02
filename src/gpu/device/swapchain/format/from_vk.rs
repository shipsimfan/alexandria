use crate::gpu::{
    VulkanSwapchainFormat, device::swapchain::format::other::OtherVulkanSwapchainFormat,
};
use vulkan::{
    VkFormat,
    khr_surface::{VkColorSpaceKhr, VkSurfaceFormatKhr},
};

impl VulkanSwapchainFormat {
    /// Convert a Vulkan surface format into a [`SwapchainFormat`]
    pub(crate) fn from_vk(vk: VkSurfaceFormatKhr) -> Option<VulkanSwapchainFormat> {
        if vk.color_space != VkColorSpaceKhr::SRGBNonlinearKhr {
            return None;
        }

        match vk.format {
            VkFormat::B8G8R8A8UNorm => Some(VulkanSwapchainFormat::B8G8R8A8Srgb),
            VkFormat::R8G8B8A8UNorm => Some(VulkanSwapchainFormat::R8G8B8A8Srgb),
            VkFormat::R4G4B4A4UNormPack16
            | VkFormat::B4G4R4A4UNormPack16
            | VkFormat::R5G6B5UNormPack16
            | VkFormat::B5G6R5UNormPack16
            | VkFormat::A1R5G5B5UNormPack16
            | VkFormat::A1B5G5R5UnormPack16
            | VkFormat::A8B8G8R8SNormPack32
            | VkFormat::A2B10G10R10UNormPack32
            | VkFormat::A2R10G10B10UNormPack32
            | VkFormat::R16G16B16A16UNorm => Some(VulkanSwapchainFormat::OtherSrgb(
                OtherVulkanSwapchainFormat::new(vk.format),
            )),
            _ => None,
        }
    }
}
