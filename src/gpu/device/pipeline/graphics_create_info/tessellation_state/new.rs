use crate::gpu::VulkanPipelineTessellationStateCreateInfo;
use vulkan::VkPipelineTessellationStateCreateInfo;

impl VulkanPipelineTessellationStateCreateInfo {
    /// Create a new [`VulkanPipelineTessellationStateCreateInfo`]
    pub fn new(patch_control_points: u32) -> VulkanPipelineTessellationStateCreateInfo {
        VulkanPipelineTessellationStateCreateInfo {
            inner: VkPipelineTessellationStateCreateInfo {
                patch_control_points,
                ..Default::default()
            },
        }
    }
}
