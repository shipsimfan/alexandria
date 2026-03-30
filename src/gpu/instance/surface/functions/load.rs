use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanSurfaceFunctions, load_instance_function},
};
use vulkan::{
    VkInstance,
    khr_surface::{
        VK_DESTROY_SURFACE_KHR, VK_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR,
        VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR,
        VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR,
    },
};

impl VulkanSurfaceFunctions {
    /// Load all the required surface functions
    pub fn load(context: &GpuSubsystem, instance: VkInstance) -> Result<VulkanSurfaceFunctions> {
        Ok(VulkanSurfaceFunctions {
            get_physical_device_surface_support: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR
            )?,
            get_physical_device_surface_present_modes: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR
            )?,
            get_physical_device_surface_formats: load_instance_function!(
                context,
                instance,
                VK_GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR
            )?,
            destroy_surface: load_instance_function!(context, instance, VK_DESTROY_SURFACE_KHR)?,
        })
    }
}
