use crate::gpu::{VulkanDevice, VulkanImage};
use vulkan::khr_swapchain::VkSwapchainKhr;

mod functions;

mod acquire_next_image;
mod drop;
mod get;
mod nameable;
mod new;

pub(in crate::gpu::device) use functions::VulkanSwapchainFunctions;

/// A swapchain used for presenting rendered images on window surfaces
pub struct VulkanSwapchain {
    /// The handle to the underlying swapchain
    handle: VkSwapchainKhr,

    /// The set of images that make up this swapchain
    images: Vec<VulkanImage>,

    /// The device that created this swapchain
    device: VulkanDevice,
}
