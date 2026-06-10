use crate::gpu::VulkanSurfaceCapabilities;
use vulkan::khr_surface::VkSurfaceCapabilitiesKhr;

impl VulkanSurfaceCapabilities {
    /// Creates a new [`VulkanSurfaceCapabilities`] from the given Vulkan surface capabilities
    pub(in crate::gpu) fn new(inner: VkSurfaceCapabilitiesKhr) -> VulkanSurfaceCapabilities {
        VulkanSurfaceCapabilities { inner }
    }
}
