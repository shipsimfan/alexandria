use vulkan::khr_surface::{VkDestroySurfaceKhr, VkGetPhysicalDeviceSurfaceSupportKhr};

#[cfg(target_os = "windows")]
mod win32;

mod load;

pub(in crate::instance) use win32::Win32WindowSurfaceFunctions;

/// Functions used by window surfaces
pub(in crate::instance) struct WindowSurfaceFunctions {
    /// Get if a physical device supports presentation on a surface using a specific queue family
    pub get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKhr,

    /// The function to destroy the surface
    pub destroy_surface: VkDestroySurfaceKhr,
}
