use crate::WindowSurface;
use vulkan::khr_surface::VkSurfaceKhr;

impl WindowSurface {
    /// Get the underlying handle to the surface
    pub(in crate::instance) fn handle(&self) -> VkSurfaceKhr {
        self.handle
    }
}
