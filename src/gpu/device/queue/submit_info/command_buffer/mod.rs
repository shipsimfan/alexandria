use std::marker::PhantomData;
use vulkan::VkCommandBufferSubmitInfo;

mod get;
mod new;
mod set;

/// The command buffer submit info for a queue submission
#[repr(transparent)]
pub struct VulkanCommandBufferSubmitInfo<'a> {
    /// The inner Vulkan command buffer submit info struct
    inner: VkCommandBufferSubmitInfo,

    /// A marker for the lifetime of the command buffer in this submit info
    _marker: PhantomData<&'a ()>,
}
