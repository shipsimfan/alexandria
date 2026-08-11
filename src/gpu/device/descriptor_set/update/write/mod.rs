use std::marker::PhantomData;
use vulkan::VkWriteDescriptorSet;

mod buffer_info;
mod image_info;

mod get;
mod new;
mod set;

pub use buffer_info::*;
pub use image_info::*;

/// An operation to write descriptor set data to a descriptor set
#[repr(transparent)]
pub struct VulkanWriteDescriptorSet<'a> {
    /// The inner Vulkan write descriptor set structure
    inner: VkWriteDescriptorSet,

    /// A marker for the lifetime of the descriptor set and the image views or buffer views
    /// involved in the write operation
    _marker: PhantomData<&'a ()>,
}
