use vulkan::khr_surface::{
    VkDestroySurfaceKhr, VkGetPhysicalDeviceSurfaceFormatsKhr,
    VkGetPhysicalDeviceSurfacePresentModesKhr, VkGetPhysicalDeviceSurfaceSupportKhr,
};

#[cfg(target_os = "linux")]
mod wayland;
#[cfg(target_os = "windows")]
mod win32;

mod load;

#[cfg(target_os = "linux")]
pub(in crate::instance) use wayland::WaylandWindowSurfaceFunctions;
#[cfg(target_os = "windows")]
pub(in crate::instance) use win32::Win32WindowSurfaceFunctions;

/// Functions used by window surfaces
pub(in crate::instance) struct WindowSurfaceFunctions {
    /// Get if a physical device supports presentation on a surface using a specific queue family
    pub get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKhr,

    /// Get the swapchain presentation modes supported by a physical device
    pub get_physical_device_surface_present_modes: VkGetPhysicalDeviceSurfacePresentModesKhr,

    /// Get the surface formats supported by a physical device
    pub get_physical_device_surface_formats: VkGetPhysicalDeviceSurfaceFormatsKhr,

    /// The function to destroy the surface
    pub destroy_surface: VkDestroySurfaceKhr,
}
