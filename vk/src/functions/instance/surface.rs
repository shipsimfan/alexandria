use crate::functions::get_instance_proc_addr;
use std::ffi::CStr;
use vulkan::{
    VkAllocationCallbacks, VkDestroySurfaceKHR, VkInstance, VkSurfaceKHR, VK_DESTROY_SURFACE_KHR,
};

/// Functions for surfaces
pub(crate) struct SurfaceFunctions {
    destroy_surface: VkDestroySurfaceKHR,
}

impl SurfaceFunctions {
    /// Loads functions for surfaces
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let destroy_surface: VkDestroySurfaceKHR =
            get_instance_proc_addr!(instance, VK_DESTROY_SURFACE_KHR)?;

        Ok(SurfaceFunctions { destroy_surface })
    }

    /// Destorys `surface`
    pub(crate) fn destroy_surface(
        &self,
        instance: VkInstance,
        surface: VkSurfaceKHR,
        allocator: *const VkAllocationCallbacks,
    ) {
        (self.destroy_surface)(instance, surface, allocator);
    }
}
