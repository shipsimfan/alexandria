use crate::{
    Result,
    gpu::{
        VulkanDevice, VulkanSurface, VulkanSwapchain, VulkanSwapchainFormat,
        VulkanSwapchainPresentMode,
    },
    math::Vector2i,
};

impl VulkanDevice {
    /// Create a new [`VulkanSwapchain`]
    pub fn create_swapchain<'surface>(
        &self,
        image_count: u32,
        image_format: VulkanSwapchainFormat,
        image_size: Vector2i,
        present_mode: VulkanSwapchainPresentMode,

        surface: &'surface VulkanSurface,
    ) -> Result<VulkanSwapchain<'surface>> {
        VulkanSwapchain::new(
            image_count,
            image_format,
            image_size,
            present_mode,
            surface,
            self.clone(),
        )
    }
}
