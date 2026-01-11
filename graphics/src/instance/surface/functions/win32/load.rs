use crate::{Result, instance::Win32WindowSurfaceFunctions, util::load_instance_function};
use vulkan::{VkInstance, khr_win32_surface::VK_CREATE_WIN32_SURFACE_KHR};

impl Win32WindowSurfaceFunctions {
    /// Load all the required Win32 surface functions
    pub fn load(instance: VkInstance) -> Result<Win32WindowSurfaceFunctions> {
        Ok(Win32WindowSurfaceFunctions {
            create_surface: load_instance_function!(instance, VK_CREATE_WIN32_SURFACE_KHR)?,
        })
    }
}
