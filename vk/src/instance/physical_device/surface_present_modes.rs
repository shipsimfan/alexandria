use std::ptr::null_mut;

use crate::{PhysicalDevice, Surface};
use vulkan::{VkPresentModeKHR, VkResult};

impl PhysicalDevice {
    /// Gets the [`VkPresentModeKHR`]s that this physical device supports on `surface`
    pub fn surface_present_modes(
        &self,
        surface: &Surface,
    ) -> Result<Vec<VkPresentModeKHR>, VkResult> {
        let present_mode_count =
            self.vk_get_physical_device_surface_present_modes(surface, null_mut(), 0)?;
        let mut present_modes = Vec::with_capacity(present_mode_count);

        let present_mode_count = self.vk_get_physical_device_surface_present_modes(
            surface,
            present_modes.as_mut_ptr(),
            present_mode_count,
        )?;
        unsafe { present_modes.set_len(present_mode_count) };

        Ok(present_modes)
    }

    /// Calls `vkGetPhysicalDeviceSurfacePresentModesKHR`
    fn vk_get_physical_device_surface_present_modes(
        &self,
        surface: &Surface,
        present_modes: *mut VkPresentModeKHR,
        present_mode_count: usize,
    ) -> Result<usize, VkResult> {
        let mut present_mode_count = present_mode_count as u32;
        surface
            .f()
            .get_physical_device_surface_present_modes(
                self.handle,
                surface.handle(),
                &mut present_mode_count,
                present_modes,
            )
            .map(|_| present_mode_count as usize)
    }
}
