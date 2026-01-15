use crate::{GraphicsAdapter, GraphicsError, Result, SwapchainPresentMode, WindowSurface};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl<'instance> GraphicsAdapter<'instance> {
    /// Get the [`SwapchainPresentMode`]s this adapter supports on `surface`
    pub fn surface_present_modes(
        &self,
        surface: &WindowSurface,
    ) -> Result<Vec<SwapchainPresentMode>> {
        let mut count = 0;
        try_vulkan!((self
            .instance
            .functions
            .surface()
            .get_physical_device_surface_present_modes)(
            self.handle,
            surface.handle(),
            &mut count,
            null_mut()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain present mode count", vk))?;

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut present_modes = Vec::with_capacity(count as _);
        try_vulkan!((self
            .instance
            .functions
            .surface()
            .get_physical_device_surface_present_modes)(
            self.handle,
            surface.handle(),
            &mut count,
            present_modes.as_mut_ptr()
        ))
        .map_err(|vk| GraphicsError::new_vk("unable to get swapchain present modes", vk))?;

        unsafe { present_modes.set_len(count as _) };

        Ok(present_modes
            .into_iter()
            .filter_map(SwapchainPresentMode::from_vk)
            .collect())
    }
}
