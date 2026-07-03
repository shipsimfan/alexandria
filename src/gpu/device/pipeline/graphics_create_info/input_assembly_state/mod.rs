use vulkan::VkPipelineInputAssemblyStateCreateInfo;

mod as_ptr;
mod get;
mod new;
mod set;

/// The state of the input assembly stage in a graphics pipeline
pub struct VulkanPipelineInputAssemblyStateCreateInfo {
    /// The inner Vulkan input assembly state create info structure
    inner: VkPipelineInputAssemblyStateCreateInfo,
}
