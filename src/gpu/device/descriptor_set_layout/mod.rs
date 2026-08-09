use crate::gpu::VulkanDevice;
use vulkan::VkDescriptorSetLayout;

mod functions;
mod layout_binding;

mod drop;
mod get;
mod new;

pub use layout_binding::*;

pub(in crate::gpu::device) use functions::*;

/// A set of resources that can be bound to a pipeline
pub struct VulkanDescriptorSetLayout {
    /// The handle to the underlying Vulkan descriptor set layout
    handle: VkDescriptorSetLayout,

    /// The device this descriptor set layout is on
    device: VulkanDevice,
}
