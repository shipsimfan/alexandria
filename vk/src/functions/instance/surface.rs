use crate::functions::get_instance_proc_addr;
use std::ffi::CStr;
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkDestroySurfaceKHR, VkGetPhysicalDeviceSurfaceSupportKHR,
    VkInstance, VkPhysicalDevice, VkSurfaceKHR, VK_DESTROY_SURFACE_KHR, VK_FALSE,
    VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR, VK_TRUE,
};

/// Functions for surfaces
pub(crate) struct SurfaceFunctions {
    get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR,
    destroy_surface: VkDestroySurfaceKHR,
}

impl SurfaceFunctions {
    /// Loads functions for surfaces
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let destroy_surface: VkDestroySurfaceKHR =
            get_instance_proc_addr!(instance, VK_DESTROY_SURFACE_KHR)?;
        let get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR)?;

        Ok(SurfaceFunctions {
            destroy_surface,
            get_physical_device_surface_support,
        })
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

    /// Checks if `device` can support presenting to `surface` on the queue family at
    /// `queue_family_index`
    pub(crate) fn get_physical_device_surface_support(
        &self,
        device: VkPhysicalDevice,
        queue_family_index: u32,
        surface: VkSurfaceKHR,
    ) -> bool {
        let mut present_support = VK_FALSE;
        try_vulkan!((self.get_physical_device_surface_support)(
            device,
            queue_family_index,
            surface,
            &mut present_support,
        ))
        .map(|_| present_support == VK_TRUE)
        .unwrap_or(false)
    }
}
