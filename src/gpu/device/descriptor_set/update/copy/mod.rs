use crate::gpu::VulkanDescriptorSet;
use std::marker::PhantomData;
use vulkan::VkCopyDescriptorSet;

mod get;
mod new;
mod set;

/// An operation to copy descriptor set data from one descriptor set to another
#[repr(transparent)]
pub struct VulkanCopyDescriptorSet<'a> {
    /// The inner Vulkan copy descriptor set structure
    inner: VkCopyDescriptorSet,

    /// A marker for the lifetime of the descriptor sets involved in the copy operation
    _marker: PhantomData<&'a VulkanDescriptorSet>,
}
