use crate::FunctionSymbol;
use vulkan::khr_surface::{
    VkDestroySurfaceKhr, VkGetPhysicalDeviceSurfaceFormatsKhr,
    VkGetPhysicalDeviceSurfacePresentModesKhr, VkGetPhysicalDeviceSurfaceSupportKhr,
};

mod win32;

mod load;

pub(in crate::gpu::instance) use win32::*;

/// Functions used by surfaces
pub(in crate::gpu::instance) struct VulkanSurfaceFunctions {
    /// Get if a physical device supports presentation on a surface using a specific queue family
    pub get_physical_device_surface_support: FunctionSymbol<VkGetPhysicalDeviceSurfaceSupportKhr>,

    /// Get the swapchain presentation modes supported by a physical device
    pub get_physical_device_surface_present_modes:
        FunctionSymbol<VkGetPhysicalDeviceSurfacePresentModesKhr>,

    /// Get the surface formats supported by a physical device
    pub get_physical_device_surface_formats: FunctionSymbol<VkGetPhysicalDeviceSurfaceFormatsKhr>,

    /// The function to destroy a surface
    pub destroy_surface: FunctionSymbol<VkDestroySurfaceKhr>,
}
