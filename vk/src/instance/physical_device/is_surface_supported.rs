use super::PhysicalDevice;
use crate::Surface;

impl PhysicalDevice {
    /// Checks if `surface` is support by the queue family on this device at `queue_family_index`
    pub fn is_surface_supported(&self, queue_family_index: usize, surface: &Surface) -> bool {
        surface.f().get_physical_device_surface_support(
            self.handle,
            queue_family_index as _,
            surface.handle(),
        )
    }
}
