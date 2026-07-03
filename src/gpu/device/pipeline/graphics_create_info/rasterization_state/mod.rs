use vulkan::VkPipelineRasterizationStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the rasterization stage in a graphics pipeline
pub struct VulkanPipelineRasterizationStateCreateInfo {
    /// The inner representation of the Vulkan pipeline rasterization state create info
    inner: VkPipelineRasterizationStateCreateInfo,
}
