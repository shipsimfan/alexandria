use std::marker::PhantomData;
use vulkan::VkCommandBufferInheritanceInfo;

mod get;
mod new;
mod set;

/// Information describing the inheritance state of a secondary Vulkan command buffer
#[repr(transparent)]
pub struct VulkanCommandBufferInheritanceInfo<'a> {
    /// The underlying Vulkan command buffer inheritance info
    inner: VkCommandBufferInheritanceInfo,

    /// A marker for the lifetimes of the contained references
    _marker: PhantomData<&'a ()>,
}
