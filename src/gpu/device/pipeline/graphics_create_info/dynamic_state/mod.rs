use std::marker::PhantomData;
use vulkan::VkPipelineDynamicStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The dynamic states of a graphics pipeline
pub struct VulkanPipelineDynamicStateCreateInfo<'a> {
    /// The inner Vulkan pipeline dynamic state create info
    inner: VkPipelineDynamicStateCreateInfo,

    /// A marker for the lifetime of the dynamic states
    _marker: PhantomData<&'a ()>,
}
