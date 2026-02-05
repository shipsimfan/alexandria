use crate::gpu::subsystem::GlobalVulkanFunctions;

mod all_extensions;
mod extensions;
mod layers;
mod new;
mod version;

/// Allows interaction and control over GPUs
pub struct GpuSubsystemInner {
    /// The Vulkan functions not specific to any Vulkan instance
    pub(in crate::gpu) functions: GlobalVulkanFunctions,
}
