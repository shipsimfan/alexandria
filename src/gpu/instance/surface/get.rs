use crate::gpu::VulkanSurface;
use vulkan::khr_surface::VkSurfaceKhr;

impl VulkanSurface {
    /// Get the underlying handle to the surface
    pub(in crate::gpu) fn handle(&self) -> VkSurfaceKhr {
        self.handle
    }
}
