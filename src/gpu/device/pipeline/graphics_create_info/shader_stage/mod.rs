use std::marker::PhantomData;
use vulkan::VkPipelineShaderStageCreateInfo;

mod specialization_info;

mod get;
mod new;
mod set;

pub use specialization_info::*;

/// Information about a shader stage in a graphics pipeline
#[repr(transparent)]
pub struct VulkanPipelineShaderStageCreateInfo<'a> {
    /// The inner representation of the creation info
    inner: VkPipelineShaderStageCreateInfo,

    /// A marker to indicate the lifetimes of borrowed data
    _marker: PhantomData<&'a ()>,
}
