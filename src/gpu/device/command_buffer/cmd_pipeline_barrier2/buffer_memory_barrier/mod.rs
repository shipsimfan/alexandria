use std::marker::PhantomData;
use vulkan::VkBufferMemoryBarrier2;

mod get;
mod new;
mod set;

/// A Vulkan buffer memory barrier used in pipeline barriers
#[repr(transparent)]
pub struct VulkanBufferMemoryBarrier<'a> {
    /// The inner Vulkan buffer memory barrier structure
    inner: VkBufferMemoryBarrier2,

    /// A marker to indicate that this buffer memory barrier borrows from a Vulkan buffer
    _marker: PhantomData<&'a ()>,
}
