use std::marker::PhantomData;
use vulkan::VkSubmitInfo2;

mod command_buffer;
mod semaphore;

mod get;
mod new;
mod set;

pub use command_buffer::*;
pub use semaphore::*;

/// The submit info for a queue submission
#[repr(transparent)]
pub struct VulkanSubmitInfo<'a> {
    /// The inner Vulkan submit info struct
    inner: VkSubmitInfo2,

    /// A marker for the lifetime of semaphores and command buffers in this submit info
    _marker: PhantomData<&'a ()>,
}
