use vulkan::VkFormat;

mod into_vk;
mod new;

/// A non-standard swapchain format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OtherSwapchainFormat {
    /// The actual Vulkan format
    format: VkFormat,
}
