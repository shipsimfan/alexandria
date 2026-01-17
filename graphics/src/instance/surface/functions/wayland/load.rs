use crate::{Result, instance::WaylandWindowSurfaceFunctions, util::load_instance_function};
use vulkan::{VkInstance, khr_wayland_surface::VK_CREATE_WAYLAND_SURFACE_KHR};

impl WaylandWindowSurfaceFunctions {
    /// Load all the required Wayland surface functions
    pub fn load(instance: VkInstance) -> Result<WaylandWindowSurfaceFunctions> {
        Ok(WaylandWindowSurfaceFunctions {
            create_surface: load_instance_function!(instance, VK_CREATE_WAYLAND_SURFACE_KHR)?,
        })
    }
}
