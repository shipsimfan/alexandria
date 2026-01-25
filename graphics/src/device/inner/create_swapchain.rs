use crate::{
    Result, Swapchain, SwapchainFormat, SwapchainPresentMode, WindowSurface,
    device::GraphicsDeviceInner,
};
use alexandria_math::Vector2u;
use std::sync::Arc;

impl GraphicsDeviceInner {
    /// Create a new [`Swapchain`]
    pub fn create_swapchain<'surface>(
        self: &Arc<GraphicsDeviceInner>,
        image_count: u32,
        image_format: SwapchainFormat,
        image_size: Vector2u,
        present_mode: SwapchainPresentMode,

        surface: &'surface WindowSurface,
    ) -> Result<Swapchain<'surface>> {
        Swapchain::new(
            image_count,
            image_format,
            image_size,
            present_mode,
            surface,
            self.clone(),
        )
    }
}
