use vulkan::VkPipelineDepthStencilStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the depth-stencil stage in a graphics pipeline
pub struct VulkanPipelineDepthStencilStateCreateInfo {
    /// The inner representation of the Vulkan pipeline depth-stencil state create info
    inner: VkPipelineDepthStencilStateCreateInfo,
}
