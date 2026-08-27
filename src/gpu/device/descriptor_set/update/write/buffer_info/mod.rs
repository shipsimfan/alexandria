use crate::gpu::VulkanBuffer;
use std::marker::PhantomData;
use vulkan::VkDescriptorBufferInfo;

mod get;
mod new;
mod set;

/// A Vulkan descriptor buffer info object
#[repr(transparent)]
pub struct VulkanDescriptorBufferInfo<'a> {
    /// The underlying Vulkan descriptor buffer info
    inner: VkDescriptorBufferInfo,

    /// A marker for the lifetimes of the buffer
    _marker: PhantomData<&'a VulkanBuffer>,
}
