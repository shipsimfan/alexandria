use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanWaylandSurfaceFunctions, load_instance_function},
};
use vulkan::{VkInstance, khr_wayland_surface::VK_CREATE_WAYLAND_SURFACE_KHR};

impl VulkanWaylandSurfaceFunctions {
    /// Load all the required Wayland surface functions
    pub fn load(
        context: &GpuSubsystem,
        instance: VkInstance,
    ) -> Result<VulkanWaylandSurfaceFunctions> {
        Ok(VulkanWaylandSurfaceFunctions {
            create_wayland_surface: load_instance_function!(
                context,
                instance,
                VK_CREATE_WAYLAND_SURFACE_KHR
            )?,
        })
    }
}
