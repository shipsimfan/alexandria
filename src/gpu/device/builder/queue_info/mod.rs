use std::marker::PhantomData;
use vulkan::VkDeviceQueueCreateInfo;

mod get;
mod new;
mod set;

/// The information describing a queue to create
#[repr(transparent)]
pub struct VulkanQueueCreateInfo<'a> {
    /// The inner Vulkan structure
    inner: VkDeviceQueueCreateInfo,

    /// A marker for the lifetime of the priorities
    _priorities: PhantomData<&'a ()>,
}
