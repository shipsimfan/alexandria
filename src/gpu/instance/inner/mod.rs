use crate::gpu::{GpuSubsystem, VulkanInstanceFunctions};
use vulkan::VkInstance;

mod create_debug_messenger;
mod drop;
mod enumerate_adapters;
mod get;
mod new;

/// The context for interacting with Vulkan
pub struct VulkanInstanceInner {
    /// A handle to the underlying graphics intance
    handle: VkInstance,

    /// The functions loaded for this instance
    pub(in crate::gpu) functions: VulkanInstanceFunctions,

    /// The context this instance came from
    _context: GpuSubsystem,
}
