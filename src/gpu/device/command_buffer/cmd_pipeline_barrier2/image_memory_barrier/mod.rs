use std::marker::PhantomData;
use vulkan::VkImageMemoryBarrier2;

mod get;
mod new;
mod set;

/// A Vulkan image memory barrier used in pipeline barriers
#[repr(transparent)]
pub struct VulkanImageMemoryBarrier<'a> {
    /// The inner Vulkan image memory barrier structure
    inner: VkImageMemoryBarrier2,

    /// A marker to indicate that this image memory barrier borrows from a Vulkan image
    _marker: PhantomData<&'a ()>,
}
