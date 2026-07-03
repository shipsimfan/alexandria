use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanSurface, VulkanSurfaceCapabilities},
};
use vulkan::{khr_surface::VkSurfaceCapabilitiesKhr, try_vulkan};

impl VulkanDevice {
    /// Get the surface capabilities of the device for the given surface
    pub fn surface_capabilities(
        &self,
        surface: &VulkanSurface,
    ) -> Result<VulkanSurfaceCapabilities> {
        let mut capabilities = VkSurfaceCapabilitiesKhr::default();
        try_vulkan!((self
            .instance()
            .functions()
            .adapter
            .get_physical_device_surface_capabilities_khr)(
            self.inner.physical_device(),
            surface.handle(),
            &mut capabilities
        ))
        .map(|_| VulkanSurfaceCapabilities::new(capabilities))
        .map_err(|error| Error::new_with("unable to query surface capabilities", error))
    }
}
