use vulkan::VkPipelineTessellationStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the tessellation stage in a graphics pipeline
pub struct VulkanPipelineTessellationStateCreateInfo {
    /// The inner Vulkan tessellation state create info structure
    inner: VkPipelineTessellationStateCreateInfo,
}
