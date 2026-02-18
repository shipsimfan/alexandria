use crate::gpu::subsystem::GlobalVulkanFunctions;

mod all_extensions;
mod get;
mod layers;
mod new;
mod version;

/// Allows interaction and control over GPUs
pub(in crate::gpu::subsystem) struct GpuSubsystemInner {
    /// The Vulkan functions not specific to any Vulkan instance
    functions: GlobalVulkanFunctions,
}
