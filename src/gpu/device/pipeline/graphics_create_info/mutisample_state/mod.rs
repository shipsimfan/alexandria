use std::marker::PhantomData;
use vulkan::VkPipelineMultisampleStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the multisample stage in a graphics pipeline
pub struct VulkanPipelineMultisampleStateCreateInfo<'a> {
    /// The inner representation of the Vulkan pipeline multisample state create info.
    inner: VkPipelineMultisampleStateCreateInfo,

    /// A marker for the lifetime of the sample mask slice
    _marker: PhantomData<&'a [u32]>,
}
