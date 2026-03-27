use crate::{
    Result,
    gpu::{GpuSubsystem, instance::VulkanWin32SurfaceFunctions, load_instance_function},
};
use vulkan::{VkInstance, khr_win32_surface::VK_CREATE_WIN32_SURFACE_KHR};

impl VulkanWin32SurfaceFunctions {
    /// Load all the required Win32 surface functions
    pub fn load(
        context: &GpuSubsystem,
        instance: VkInstance,
    ) -> Result<VulkanWin32SurfaceFunctions> {
        Ok(VulkanWin32SurfaceFunctions {
            create_win32_surface: load_instance_function!(
                context,
                instance,
                VK_CREATE_WIN32_SURFACE_KHR
            )?,
        })
    }
}
