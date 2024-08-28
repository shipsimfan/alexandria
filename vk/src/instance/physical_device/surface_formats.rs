use crate::{PhysicalDevice, Surface};
use std::ptr::null_mut;
use vulkan::{VkResult, VkSurfaceFormatKHR};

impl PhysicalDevice {
    /// Gets the [`VkSurfaceFormatKHR`]s that this physical devices supports on `surface`
    pub fn surface_formats(&self, surface: &Surface) -> Result<Vec<VkSurfaceFormatKHR>, VkResult> {
        let format_count = self.vk_get_physical_device_surface_formats(surface, null_mut(), 0)?;
        let mut formats = Vec::with_capacity(format_count);

        let format_count = self.vk_get_physical_device_surface_formats(
            surface,
            formats.as_mut_ptr(),
            format_count,
        )?;
        unsafe { formats.set_len(format_count) };

        Ok(formats)
    }

    /// Calls `vkGetPhysicalDeviceSurfaceFormatsKHR`
    fn vk_get_physical_device_surface_formats(
        &self,
        surface: &Surface,
        formats: *mut VkSurfaceFormatKHR,
        format_count: usize,
    ) -> Result<usize, VkResult> {
        let mut format_count = format_count as u32;
        surface
            .f()
            .get_physical_device_surface_formats(
                self.handle,
                surface.handle(),
                &mut format_count,
                formats,
            )
            .map(|_| format_count as usize)
    }
}
