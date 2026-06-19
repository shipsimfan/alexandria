use vulkan::VkPipelineRasterizationStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the rasterizer stage in a graphics pipeline
pub struct VulkanPipelineRasterizerStateCreateInfo {
    /// The inner representation of the Vulkan pipeline rasterizer state create info.
    inner: VkPipelineRasterizationStateCreateInfo,
}
