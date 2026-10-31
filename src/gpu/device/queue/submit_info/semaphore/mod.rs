use std::marker::PhantomData;
use vulkan::VkSemaphoreSubmitInfo;

mod timeline;

mod get;
mod new;
mod set;

pub use timeline::*;

/// The semaphore submit info for a queue submission
#[repr(transparent)]
pub struct VulkanSemaphoreSubmitInfo<'a> {
    /// The inner Vulkan structure
    inner: VkSemaphoreSubmitInfo,

    /// Marker to tie the lifetime of the semaphore to this struct
    _marker: PhantomData<&'a ()>,
}
