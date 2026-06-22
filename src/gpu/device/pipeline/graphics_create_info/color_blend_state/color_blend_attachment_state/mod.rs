use vulkan::VkPipelineColorBlendAttachmentState;

mod get;
mod new;
mod set;

/// A description of how color blending is performed for a single color attachment
#[repr(transparent)]
pub struct VulkanPipelineColorBlendAttachmentState {
    /// The inner representation of the Vulkan pipeline color blend attachment state
    inner: VkPipelineColorBlendAttachmentState,
}
