use crate::{
    Result, SharedObject,
    gpu::{subsystem::GlobalVulkanFunctions, subsystem::GpuSubsystemInner},
};

#[cfg(target_os = "windows")]
const VULKAN_LIBRARY_NAME: &str = "vulkan-1.dll";
#[cfg(target_os = "linux")]
const VULKAN_LIBRARY_NAME: &str = "libvulkan.so.1";

impl GpuSubsystemInner {
    /// Create a new [`GpuSubsystemInner`]
    pub(in crate::gpu::subsystem) fn new() -> Result<GpuSubsystemInner> {
        let vulkan_library = SharedObject::open(VULKAN_LIBRARY_NAME)?;
        let functions = GlobalVulkanFunctions::load(&vulkan_library)?;

        Ok(GpuSubsystemInner { functions })
    }
}
