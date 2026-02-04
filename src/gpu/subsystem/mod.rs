use crate::SharedObject;
use functions::GlobalVulkanFunctions;

mod functions;

mod new;
mod version;

/// Allows interaction and control over GPUs
pub struct GpuSubsystem {
    /// The library to get Vulkan functions from
    vulkan_library: SharedObject,

    /// The Vulkan functions not specific to any Vulkan instance
    functions: GlobalVulkanFunctions,
}
