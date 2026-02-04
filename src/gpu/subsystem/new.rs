use crate::{
    AlexandriaContextInner, Result, SharedObject,
    gpu::{GpuSubsystem, subsystem::GlobalVulkanFunctions},
};
use std::{mem::MaybeUninit, sync::Weak};

#[cfg(target_os = "windows")]
const VULKAN_LIBRARY_NAME: &str = "vulkan-1.dll";
#[cfg(target_os = "linux")]
const VULKAN_LIBRARY_NAME: &str = "libvulkan.so.1";

impl GpuSubsystem {
    /// Create a new [`GpuSubsystem`]
    pub(crate) fn new(context: Weak<MaybeUninit<AlexandriaContextInner>>) -> Result<GpuSubsystem> {
        let vulkan_library = SharedObject::open(VULKAN_LIBRARY_NAME)?;
        let functions = GlobalVulkanFunctions::load(&vulkan_library)?;

        Ok(GpuSubsystem { context, functions })
    }
}
