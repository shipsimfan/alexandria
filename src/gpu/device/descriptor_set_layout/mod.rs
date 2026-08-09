use crate::gpu::VulkanDevice;
use vulkan::VkDescriptorSetLayout;

mod functions;

mod drop;
mod get;

pub(in crate::gpu::device) use functions::*;

/// A set of resources that can be bound to a pipeline
pub struct VulkanDescriptorSetLayout {
    /// The handle to the underlying Vulkan descriptor set layout
    handle: VkDescriptorSetLayout,

    /// The device this descriptor set layout is on
    device: VulkanDevice,
}
