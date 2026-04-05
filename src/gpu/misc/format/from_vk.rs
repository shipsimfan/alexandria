use crate::gpu::VulkanFormat;
use vulkan::{
    VkFormat,
    khr_surface::{VkColorSpaceKhr, VkSurfaceFormatKhr},
};

impl VulkanFormat {
    /// Convert a Vulkan surface format into a [`VulkanFormat`]
    pub(in crate::gpu) fn from_vk_surface_format(vk: VkSurfaceFormatKhr) -> Option<VulkanFormat> {
        if vk.color_space != VkColorSpaceKhr::SRGBNonlinearKhr {
            return None;
        }

        match vk.format {
            VkFormat::B8G8R8A8UNorm => Some(VulkanFormat::B8G8R8A8UNorm),
            VkFormat::R8G8B8A8UNorm => Some(VulkanFormat::R8G8B8A8UNorm),
            _ => None,
        }
    }
}
