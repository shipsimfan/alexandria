use vulkan::VkDescriptorSet;

mod functions;
mod update;

mod get;
mod new;

pub use update::*;

pub(in crate::gpu::device) use functions::*;

/// A Vulkan descriptor set, which is a collection of resources (like buffers and images) that can
/// be bound to a pipeline for rendering or compute operations
pub struct VulkanDescriptorSet {
    /// The handle to the underlying Vulkan descriptor set
    handle: VkDescriptorSet,
}
