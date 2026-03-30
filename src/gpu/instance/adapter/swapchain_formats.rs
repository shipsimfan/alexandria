use crate::{
    Error, Result,
    gpu::{SwapchainFormat, VulkanAdapter, VulkanSurface},
};
use std::ptr::null_mut;
use vulkan::try_vulkan;

impl<'instance> VulkanAdapter<'instance> {
    /// Get the [`SwapchainFormat`]s that this adapter supports on `surface`
    pub fn swapchain_formats(&self, surface: &VulkanSurface) -> Result<Vec<SwapchainFormat>> {
        let mut count = 0;
        try_vulkan!((self
            .instance
            .functions()
            .surface()
            .get_physical_device_surface_formats)(
            self.handle,
            surface.handle(),
            &mut count,
            null_mut()
        ))
        .map_err(|vk| Error::new_with("unable to get swapchain format count", vk))?;

        if count == 0 {
            return Ok(Vec::new());
        }

        let mut formats = Vec::with_capacity(count as _);
        try_vulkan!((self
            .instance
            .functions()
            .surface()
            .get_physical_device_surface_formats)(
            self.handle,
            surface.handle(),
            &mut count,
            formats.as_mut_ptr()
        ))
        .map_err(|vk| Error::new_with("unable to get swapchain formats", vk))?;

        unsafe { formats.set_len(count as _) };

        Ok(formats
            .into_iter()
            .filter_map(SwapchainFormat::from_vk)
            .collect())
    }
}
