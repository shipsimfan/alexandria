use crate::functions::get_instance_proc_addr;
use std::ffi::CStr;
use vulkan::{
    try_vulkan, VkAllocationCallbacks, VkDestroySurfaceKHR,
    VkGetPhysicalDeviceSurfaceCapabilitiesKHR, VkGetPhysicalDeviceSurfaceFormatsKHR,
    VkGetPhysicalDeviceSurfacePresentModesKHR, VkGetPhysicalDeviceSurfaceSupportKHR, VkInstance,
    VkPhysicalDevice, VkPresentModeKHR, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
    VkSurfaceKHR, VK_DESTROY_SURFACE_KHR, VK_FALSE,
    VK_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR, VK_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR,
    VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR, VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR,
    VK_TRUE,
};

/// Functions for surfaces
pub(crate) struct SurfaceFunctions {
    get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR,
    get_physical_device_surface_capabilities: VkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    get_physical_device_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR,
    get_physical_device_surface_present_modes: VkGetPhysicalDeviceSurfacePresentModesKHR,
    destroy_surface: VkDestroySurfaceKHR,
}

impl SurfaceFunctions {
    /// Loads functions for surfaces
    pub(super) fn new(instance: VkInstance) -> Result<Self, &'static CStr> {
        let get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR)?;
        let get_physical_device_surface_capabilities: VkGetPhysicalDeviceSurfaceCapabilitiesKHR =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR)?;
        let get_physical_device_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKHR =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR)?;
        let get_physical_device_surface_present_modes: VkGetPhysicalDeviceSurfacePresentModesKHR =
            get_instance_proc_addr!(instance, VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR)?;
        let destroy_surface: VkDestroySurfaceKHR =
            get_instance_proc_addr!(instance, VK_DESTROY_SURFACE_KHR)?;

        Ok(SurfaceFunctions {
            destroy_surface,
            get_physical_device_surface_support,
            get_physical_device_surface_capabilities,
            get_physical_device_surface_formats,
            get_physical_device_surface_present_modes,
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

    /// Gets the capabilities of `physical_device` on `surface`
    pub(crate) fn get_physical_device_surface_capabilities(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> Result<VkSurfaceCapabilitiesKHR, VkResult> {
        let mut capabilities = VkSurfaceCapabilitiesKHR::default();
        try_vulkan!((self.get_physical_device_surface_capabilities)(
            physical_device,
            surface,
            &mut capabilities
        ))
        .map(|_| capabilities)
    }

    /// Gets the list of [`VkSurfaceFormatKHR`]s that `physical_device` supports on `surface`
    pub(crate) fn get_physical_device_surface_formats(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        surface_format_count: &mut u32,
        surface_formats: *mut VkSurfaceFormatKHR,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.get_physical_device_surface_formats)(
            physical_device,
            surface,
            surface_format_count,
            surface_formats
        ))
        .map(|_| ())
    }

    /// Gets the [`VkPresentModeKHR`]s that `physical_device` supports on `surface`
    pub(crate) fn get_physical_device_surface_present_modes(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        present_mode_count: &mut u32,
        present_modes: *mut VkPresentModeKHR,
    ) -> Result<(), VkResult> {
        try_vulkan!((self.get_physical_device_surface_present_modes)(
            physical_device,
            surface,
            present_mode_count,
            present_modes
        ))
        .map(|_| ())
    }
}
