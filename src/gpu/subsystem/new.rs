use crate::{
    Result, SharedObject,
    gpu::{GpuSubsystem, subsystem::GlobalVulkanFunctions},
};

#[cfg(target_os = "windows")]
const VULKAN_LIBRARY_NAME: &str = "vulkan-1.dll";
#[cfg(target_os = "linux")]
const VULKAN_LIBRARY_NAME: &str = "libvulkan.so.1";

impl GpuSubsystem {
    /// Create a new [`GpuSubsystem`]
    pub(crate) fn new() -> Result<GpuSubsystem> {
        let vulkan_library = SharedObject::open(VULKAN_LIBRARY_NAME)?;
        let functions = GlobalVulkanFunctions::load(&vulkan_library)?;

        Ok(GpuSubsystem {
            vulkan_library,
            functions,
        })
    }
}
