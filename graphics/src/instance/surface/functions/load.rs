use crate::{Result, instance::WindowSurfaceFunctions, util::load_instance_function};
use vulkan::{
    VkInstance,
    khr_surface::{VK_DESTROY_SURFACE_KHR, VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR},
};

impl WindowSurfaceFunctions {
    /// Load all the required surface functions
    pub fn load(instance: VkInstance) -> Result<WindowSurfaceFunctions> {
        Ok(WindowSurfaceFunctions {
            get_physical_device_surface_support: load_instance_function!(
                instance,
                VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR
            )?,
            destroy_surface: load_instance_function!(instance, VK_DESTROY_SURFACE_KHR)?,
        })
    }
}
