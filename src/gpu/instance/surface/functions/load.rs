use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanSurfaceFunctions, load_instance_function},
};
use vulkan::{VkInstance, khr_surface::VK_DESTROY_SURFACE_KHR};

impl VulkanSurfaceFunctions {
    /// Load all the required surface functions
    pub fn load(context: &GpuSubsystem, instance: VkInstance) -> Result<VulkanSurfaceFunctions> {
        Ok(VulkanSurfaceFunctions {
            destroy_surface: load_instance_function!(context, instance, VK_DESTROY_SURFACE_KHR)?,
        })
    }
}
