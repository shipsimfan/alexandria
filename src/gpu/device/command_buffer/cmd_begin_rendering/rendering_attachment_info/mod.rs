use std::marker::PhantomData;
use vulkan::VkRenderingAttachmentInfo;

mod clear_value;

mod get;
mod new;
mod set;

pub use clear_value::*;

/// Information about a rendering attachment used in dynamic rendering
#[repr(transparent)]
pub struct VulkanRenderingAttachmentInfo<'a> {
    /// The inner Vulkan structure representing the rendering attachment info
    inner: VkRenderingAttachmentInfo,

    /// A marker for the lifetime of the image view reference
    _marker: PhantomData<&'a ()>,
}
