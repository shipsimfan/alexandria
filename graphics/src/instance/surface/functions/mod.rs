use vulkan::khr_surface::VkDestroySurfaceKhr;

#[cfg(target_os = "windows")]
mod win32;

mod load;

pub(in crate::instance) use win32::Win32WindowSurfaceFunctions;

/// Functions used by window surfaces
pub(in crate::instance) struct WindowSurfaceFunctions {
    /// The function to destroy the surface
    pub destroy_surface: VkDestroySurfaceKhr,
}
