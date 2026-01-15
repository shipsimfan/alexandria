use crate::{GraphicsAdapter, GraphicsError, Result, SwapchainFormat, WindowSurface};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl<'instance> GraphicsAdapter<'instance> {
    /// Get the [`SwapchainFormat`]s that this adapter supports on `surface`
    pub fn swapchain_formats(&self, surface: &WindowSurface) -> Result<Vec<SwapchainFormat>> {
        let mut count = 0;
        try_vulkan!((self
            .instance
            .functions
            .surface()
            .get_physical_device_surface_formats)(
            self.handle,
            surface.handle(),
            &mut count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain format count", vk))?;

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut present_modes = Vec::with_capacity(count as _);
        try_vulkan!((self
            .instance
            .functions
            .surface()
            .get_physical_device_surface_formats)(
            self.handle,
            surface.handle(),
            &mut count,
            present_modes.as_mut_ptr()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain formats", vk))?;

        unsafe { present_modes.set_len(count as _) };

        Ok(present_modes
            .into_iter()
            .filter_map(SwapchainFormat::from_vk)
            .collect())
    }
}
