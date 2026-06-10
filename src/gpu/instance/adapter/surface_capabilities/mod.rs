use vulkan::khr_surface::VkSurfaceCapabilitiesKhr;

mod get;
mod new;

/// The surface capabilities of an adapter for a given surface
pub struct VulkanSurfaceCapabilities {
    /// The underlying Vulkan surface capabilities
    inner: VkSurfaceCapabilitiesKhr,
}
