use crate::gpu::VulkanDevice;
use vulkan::VkDescriptorPool;

mod functions;
mod size;

mod allocate_descriptor_set;
mod drop;
mod free_descriptor_set;
mod get;
mod new;

pub use size::*;

pub(in crate::gpu::device) use functions::*;

/// A pool of Vulkan descriptor sets that can be allocated and freed as needed
pub struct VulkanDescriptorPool {
    /// The handle to the underlying Vulkan descriptor pool
    handle: VkDescriptorPool,

    /// The device this descriptor pool is on
    device: VulkanDevice,
}
