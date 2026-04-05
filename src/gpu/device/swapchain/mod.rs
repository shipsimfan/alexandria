use crate::gpu::{VulkanDevice, VulkanImage, VulkanSurface};
use vulkan::khr_swapchain::VkSwapchainKhr;

mod functions;
mod present_mode;

mod drop;
mod get;
mod new;

pub use present_mode::*;

pub(in crate::gpu::device) use functions::VulkanSwapchainFunctions;

/// A swapchain used for presenting rendered images on window surfaces
pub struct VulkanSwapchain<'surface> {
    /// The handle to the underlying swapchain
    handle: VkSwapchainKhr,

    /// The set of images that make up this swapchain
    images: Vec<VulkanImage>,

    /// The surface this swapchain was created for
    _surface: &'surface VulkanSurface,

    /// The device that created this swapchain
    device: VulkanDevice,
}
