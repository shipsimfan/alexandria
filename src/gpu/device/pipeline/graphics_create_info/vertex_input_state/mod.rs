use std::marker::PhantomData;
use vulkan::VkPipelineVertexInputStateCreateInfo;

mod attribute_description;
mod binding_description;

mod as_ptr;
mod get;
mod new;
mod set;

pub use attribute_description::*;
pub use binding_description::*;

/// The state of the vertex input stage in a graphics pipeline
pub struct VulkanPipelineVertexInputStateCreateInfo<'a> {
    /// The inner Vulkan pipeline vertex input state create info
    inner: VkPipelineVertexInputStateCreateInfo,

    /// A marker for the lifetime of the attribute and binding descriptions
    _marker: PhantomData<&'a ()>,
}
