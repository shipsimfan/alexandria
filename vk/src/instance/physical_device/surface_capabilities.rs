use crate::{PhysicalDevice, Surface};
use vulkan::{VkResult, VkSurfaceCapabilitiesKHR};

impl PhysicalDevice {
    /// Gets the capabilities this physical device supports on `surface`
    pub fn surface_capabilities(
        &self,
        surface: &Surface,
    ) -> Result<VkSurfaceCapabilitiesKHR, VkResult> {
        surface
            .f()
            .get_physical_device_surface_capabilities(self.handle, surface.handle())
    }
}
