use std::marker::PhantomData;
use vulkan::VkTimelineSemaphoreSubmitInfo;

mod get;
mod new;
mod set;

/// The information required for submitting a timeline semaphore to a Vulkan queue
#[repr(transparent)]
pub struct VulkanTimelineSemaphoreSubmitInfo<'a> {
    /// The inner Vulkan structure
    inner: VkTimelineSemaphoreSubmitInfo,

    /// Marker to tie the lifetime of the numbers contained in this struct
    _marker: PhantomData<&'a ()>,
}
