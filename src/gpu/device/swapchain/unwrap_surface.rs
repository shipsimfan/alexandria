use crate::gpu::{VulkanSurface, VulkanSwapchain};

impl<'surface> VulkanSwapchain<'surface> {
    /// Unwrap the surface reference from this swapchain, if it exists
    pub fn unwrap_surface(mut self) -> &'surface mut VulkanSurface {
        self.surface.take().unwrap()
    }
}
